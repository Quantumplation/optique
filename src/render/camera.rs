use enum_dispatch::enum_dispatch;

use crate::geometry::Bounds2;

use super::Film;

#[enum_dispatch]
pub trait Camera {
    fn bounds(&self) -> Bounds2<i32>;
}

#[enum_dispatch(Camera)]
pub enum CameraInstance {
  NullCamera,
  PerspectiveCamera,
}

pub struct NullCamera {}

impl Camera for NullCamera {
  fn bounds(&self) -> Bounds2<i32> {
    Bounds2::default()
  }
}

pub struct PerspectiveCamera {
  pub film: Film,
}

impl Camera for PerspectiveCamera {
  fn bounds(&self) -> Bounds2<i32> {
    self.film.bounds()
  }
}