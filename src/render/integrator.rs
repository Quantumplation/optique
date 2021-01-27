use std::sync::Arc;

use bumpalo::Bump;
use enum_dispatch::enum_dispatch;
use crate::{geometry::{Intersection, Point2, Ray, RayDifferential}, scene::{Light, Scene}};

use super::{BSDF, BxDFCategory, Camera, CameraInstance, RadianceProblems, Sampler, SamplerInstance, Spectrum};

#[enum_dispatch]
pub trait Integrator {
  fn render(&mut self, scene: &Scene);
}

#[enum_dispatch(Integrator)]
pub enum IntegratorInstance {
  NullIntegrator,
  SamplerIntegratorInstance
}

pub struct NullIntegrator {}

impl Integrator for NullIntegrator {
  fn render(&mut self, _scene: &Scene) {
    
  }
}

#[enum_dispatch]
pub trait SamplerIntegrator {
  fn preprocess(&mut self, scene: &Scene);
  fn light_along_ray(&self, rd: RayDifferential, scene: &Scene, sampler: &SamplerInstance, arena: &Bump, depth: u32) -> Spectrum;
  fn specular_reflect(
    &self,
    rd: RayDifferential,
    intersection: Intersection,
    bsdf: &BSDF,
    scene: &Scene,
    sampler: &SamplerInstance,
    arena: &Bump,
    depth: u32
  ) -> Spectrum {
    let outgoing = intersection.outgoing;
    let sample = bsdf.sample_function(outgoing, &Point2::new(0.5, 0.5), BxDFCategory::REFLECTION | BxDFCategory::SPECULAR);

    let normal = intersection.shading_normal;
    let incoming = sample.incoming;
    let pdf = sample.probability_distribution;
    let color_sample = sample.value;

    let factor = incoming.dot(normal.into()).abs();
    if pdf > 0. && !color_sample.is_black() && factor != 0. {
      let rd = RayDifferential {
        ray: Ray { origin: intersection.point, direction: incoming, time_max: rd.ray.time_max },
        ray_x: rd.ray_x, // TODO: compute these
        ray_y: rd.ray_y,
      };
      return color_sample * self.light_along_ray(rd, scene, sampler, arena, depth + 1) * factor / pdf;
    }
    return Spectrum::default();
  }
  fn specular_transmit(&self, rd: RayDifferential, intersection: Intersection, scene: &Scene, sampler: &SamplerInstance, arena: &Bump, depth: u32) -> Spectrum;

  fn get_camera(&mut self) -> Arc<CameraInstance>;
  fn get_sampler(&self, seed: u64) -> SamplerInstance;
}

impl<T: SamplerIntegrator> Integrator for T {
  fn render(&mut self, scene: &Scene) {
    self.preprocess(scene);
    // TODO: Parallel tiles

    // We use a bump arena to efficiently drop temporary allocations on the floor
    let mut arena = Bump::new();

    let mut sampler = self.get_sampler(0);
    let camera = self.get_camera();
    let film = camera.film();
    let bounds = camera.bounds();
    for pixel in bounds {
      sampler.start_pixel(&pixel);
      loop {
        // Choose a random ray to project along
        let camera_sample = sampler.get_camera_sample(pixel);
        let (weight, mut ray) = camera.generate_ray_differential(&camera_sample);
        
        // Scale the ray differential offsets down the more samples we're taking per pixel
        let factor = 1. / (sampler.samples_per_pixel() as f64).sqrt();
        ray.scale(factor);

        // Sample light along the ray
        let l = if weight > 0. { self.light_along_ray(ray, scene, &sampler, &arena, 0) } else { Spectrum::default() };
        use RadianceProblems::*;
        let l = match l.is_valid() {
          Some(HasNaNs) => {
            println!("Not-A-Number radiance for pixel {:?}, setting pixel to black.", pixel);
            Spectrum::default()
          },
          Some(NegativeLuminance) => {
            println!("Negative luminance for pixel {:?}, setting pixel to black.", pixel);
            Spectrum::default()
          },
          Some(InfiniteLuminance) => {
            println!("Infinite luminance for pixel {:?}, setting pixel to black.", pixel);
            Spectrum::default()
          },
          None => l,
        };

        // And mix that sample onto our film
        film.add_sample(pixel, l, weight);

        // Reset the arena for the next round
        arena.reset();
        // Break if the sampler is done
        if !sampler.start_next() {
          break;
        }
      }
    }
  }
}

#[enum_dispatch(SamplerIntegrator)]
pub enum SamplerIntegratorInstance {
  WhittedIntegrator
}

pub struct WhittedIntegrator {
  pub max_depth: u32,
  pub camera: Arc<CameraInstance>,
  pub sampler: SamplerInstance,
}
impl WhittedIntegrator {
  pub fn new(max_depth: u32, camera: CameraInstance, sampler: SamplerInstance) -> Self {
    Self { max_depth, camera: Arc::new(camera), sampler }
  }
}

impl SamplerIntegrator for WhittedIntegrator {
  fn preprocess(&mut self, _scene: &Scene) {
  }

  fn light_along_ray(&self, rd: RayDifferential, scene: &Scene, sampler: &SamplerInstance, arena: &Bump, depth: u32) -> Spectrum {

    let mut result = Spectrum::default();

    let interaction = scene.intersect(&rd.ray);
    if interaction.is_none() {
      // Since we didn't hit anything, add the background radiance from each light
      // This lets us add ambient lighting effects
      for light in &scene.lights {
        result += light.background_radiance(&rd.ray);
      }
      return result;
    }

    let interaction = interaction.unwrap();
    let outgoing = interaction.intersection.outgoing;
    // If we hit something, it might be emitting it's own light, so add in that contribution
    result += interaction.emitted_radiance();

    let bsdf = interaction.compute_scattering_functions(arena, crate::scene::TransportMode::Radiance, false);

    let bsdf = if let Some(bsdf) = bsdf {
      bsdf
    } else {
      return result;
    };

    // Now add in the contribution from each light source
    for light in &scene.lights {
      // TODO: pick a point to sample
      let radiance_sample = light.sample_radiance(&interaction.intersection, Point2::<f64>::default());
      if radiance_sample.color.is_black() || radiance_sample.probability_distribution == 0. {
        continue;
      }
      let incoming = radiance_sample.incident_direction;
      
      let value = bsdf.evaluate(outgoing, incoming, BxDFCategory::ALL);

      if !value.is_black() {
        let occlusion_ray = &radiance_sample.intersections.0.ray_between(&radiance_sample.intersections.1);
        let occluded = scene.any_intersect(occlusion_ray);
        if !occluded {
          let contribution = incoming.dot(interaction.intersection.shading_normal.into()).abs() / radiance_sample.probability_distribution;
          result += value * radiance_sample.color * contribution;
        }
      }
    }

    // And trace more bounces
    if depth + 1 < self.max_depth {
      result += self.specular_reflect(rd, interaction.intersection, bsdf, scene, sampler, arena, depth);
    }

    return result;
  }

  fn specular_transmit(&self, _rd: RayDifferential, _intersection: Intersection, _scene: &Scene, _sampler: &SamplerInstance, _arena: &Bump, _depth: u32) -> Spectrum {
    todo!()
  }

  fn get_camera(&mut self) -> Arc<CameraInstance> { self.camera.clone() }
  fn get_sampler(&self, _: u64) -> SamplerInstance { self.sampler.clone() }
}