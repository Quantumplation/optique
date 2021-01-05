use super::Scene;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Light {
  fn preprocess(&mut self, scene: &Scene);
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
  fn preprocess(&mut self, scene: &Scene) {}
}
