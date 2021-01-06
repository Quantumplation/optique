use enum_dispatch::enum_dispatch;
use crate::{geometry::SurfaceInteraction, scene::Scene};

use super::{CameraInstance, SamplerInstance};

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
  fn preprocess(&mut self, scene: &Scene, sampler: &SamplerInstance);
  fn light_along_ray(&self, /* RayDifferential, */ scene: &Scene, sampler: &SamplerInstance, /* Memory Arena, */ depth: u32) /* -> Spectrum */;
  fn specular_reflect(&self, /* RayDifferential, */ surface_interaction: SurfaceInteraction, scene: &Scene, sampler: &SamplerInstance, /* Memory Arena, */ depth: u32) /* -> Spectrum */;
  fn specular_transmit(&self, /* RayDifferential, */ surface_interaction: SurfaceInteraction, scene: &Scene, sampler: &SamplerInstance, /* Memory Arena, */ depth: u32) /* -> Spectrum */;

  fn get_camera(&self) -> &CameraInstance;
  fn get_sampler(&self) -> &SamplerInstance;
}

impl<T: SamplerIntegrator> Integrator for T {
  fn render(&mut self, scene: &Scene) {
    self.preprocess(scene, self.get_sampler());
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
  fn preprocess(&mut self, _scene: &Scene, sampler: &SamplerInstance) {
    println!("Whitted preprocess");
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