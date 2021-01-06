use enum_dispatch::enum_dispatch;

use crate::geometry::Bounds2;

use super::Film;

#[enum_dispatch]
pub trait Camera {
    fn bounds(&self) -> Bounds2<i32>;
    fn film(&mut self) -> &mut Film;
}

#[enum_dispatch(Camera)]
pub enum CameraInstance {
  PerspectiveCamera,
}

pub struct PerspectiveCamera {
  pub film: Film,
}

impl Camera for PerspectiveCamera {
  fn bounds(&self) -> Bounds2<i32> {
    self.film.bounds()
  }
  fn film(&mut self) -> &mut Film {
    &mut self.film
  }
}