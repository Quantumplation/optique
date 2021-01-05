use enum_dispatch::enum_dispatch;
use crate::scene::Scene;

#[enum_dispatch]
pub trait Integrator {
  fn render(&self, scene: &Scene);
}

#[enum_dispatch(Integrator)]
pub enum IntegratorInstance {
  NullIntegrator,
}

pub struct NullIntegrator {}

impl Integrator for NullIntegrator {
  fn render(&self, _scene: &Scene) {
    
  }
}