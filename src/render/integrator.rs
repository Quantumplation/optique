use bumpalo::Bump;
use enum_dispatch::enum_dispatch;
use crate::{geometry::SurfaceInteraction, scene::Scene};

use super::{Camera, CameraInstance, Sampler, SamplerInstance};

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
  fn light_along_ray(&self, /* RayDifferential, */ scene: &Scene, sampler: &SamplerInstance, /* Memory Arena, */ depth: u32) /* -> Spectrum */;
  fn specular_reflect(&self, /* RayDifferential, */ surface_interaction: SurfaceInteraction, scene: &Scene, sampler: &SamplerInstance, /* Memory Arena, */ depth: u32) /* -> Spectrum */;
  fn specular_transmit(&self, /* RayDifferential, */ surface_interaction: SurfaceInteraction, scene: &Scene, sampler: &SamplerInstance, /* Memory Arena, */ depth: u32) /* -> Spectrum */;

  fn get_camera(&mut self) -> &mut CameraInstance;
  fn get_sampler(&self, seed: u64) -> SamplerInstance;
}

impl<T: SamplerIntegrator> Integrator for T {
  fn render(&mut self, scene: &Scene) {
    self.preprocess(scene);
    // TODO: Parallel tiles

    let mut arena = Bump::new();
    let mut sampler = self.get_sampler(0);
    let camera = self.get_camera();
    let bounds = camera.bounds();
    let film = camera.film();
    for pixel in bounds {
      sampler.start_pixel(&pixel);
      loop {


        film.add_sample(pixel, pixel.x as f32 / bounds.max.x as f32, 1.0);
        arena.reset();
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
  pub camera: CameraInstance,
  pub sampler: SamplerInstance,
}
impl WhittedIntegrator {
  pub fn new(camera: CameraInstance, sampler: SamplerInstance) -> Self {
    Self { camera, sampler }
  }
}

impl SamplerIntegrator for WhittedIntegrator {
  fn preprocess(&mut self, _scene: &Scene) {
  }

  fn light_along_ray(&self, /* RayDifferential, */ scene: &Scene, sampler: &SamplerInstance, /* Memory Arena, */ depth: u32) {
    todo!()
  }

  fn specular_reflect(&self, /* RayDifferential, */ surface_interaction: SurfaceInteraction, scene: &Scene, sampler: &SamplerInstance, /* Memory Arena, */ depth: u32) {
    todo!()
  }

  fn specular_transmit(&self, /* RayDifferential, */ surface_interaction: SurfaceInteraction, scene: &Scene, sampler: &SamplerInstance, /* Memory Arena, */ depth: u32) {
    todo!()
  }

  fn get_camera(&mut self) -> &mut CameraInstance { &mut self.camera }
  fn get_sampler(&self, _: u64) -> SamplerInstance { self.sampler.clone() }
}