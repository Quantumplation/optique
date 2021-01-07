use crate::{geometry::Ray, render::Spectrum};

use super::Scene;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Light {
  fn preprocess(&mut self, scene: &Scene);
  fn background_radiance(&self, ray: &Ray) -> Spectrum;
}

#[enum_dispatch(Light)]
pub enum LightInstance {
  NullLight,
}

impl From<&pbrt_rs::Light> for LightInstance {
    fn from(_: &pbrt_rs::Light) -> Self {
      LightInstance::from(NullLight {})
    }
}

pub struct NullLight {}
impl Light for NullLight {
  fn preprocess(&mut self, _scene: &Scene) {}
  fn background_radiance(&self, ray: &Ray) -> Spectrum { Spectrum::default() }
}
