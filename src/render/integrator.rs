use bumpalo::Bump;
use enum_dispatch::enum_dispatch;
use crate::{geometry::SurfaceInteraction, scene::Scene};

use super::{Camera, CameraInstance, SamplerInstance};

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

  fn get_camera(&self) -> &CameraInstance;
  fn get_sampler(&self) -> &SamplerInstance;
}

impl<T: SamplerIntegrator> Integrator for T {
  fn render(&mut self, scene: &Scene) {
    self.preprocess(scene);
    // TODO: Parallel tiles

    // Allocate a memory arena for the image
    let bounds = self.get_camera().bounds();
    let mut arena = Bump::new();
    for pixel in bounds {
      println!("{:?}", pixel);
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

  fn get_camera(&self) -> &CameraInstance { &self.camera }
  fn get_sampler(&self) -> &SamplerInstance { &self.sampler }
}