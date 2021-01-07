use std::sync::Arc;

use enum_dispatch::enum_dispatch;

use crate::geometry::{Bounds2, Point3, Ray, RayDifferential, Vector3};

use super::{CameraSample, Film};

#[enum_dispatch]
pub trait Camera {
    fn bounds(&self) -> Bounds2<i32>;
    fn film(&self) -> Arc<Film>;
    fn generate_ray(&self, sample: &CameraSample) -> (f32, Ray);

    fn generate_ray_differential(&self, sample: &CameraSample) -> (f32, RayDifferential) {
      // Generate 3 rays:
      // - the one we'd normally generate
      let (wt, ray) = self.generate_ray(sample);
      
      // - shifted by one pixel in the x direction
      let (wtx, ray_x) = {
        let mut sample = sample.clone();
        sample.film_point.x += 1.;
        self.generate_ray(&sample)
      };

      // - shifted by one pixel in the y direction
      let (wty, ray_y) = {
        let mut sample = sample.clone();
        sample.film_point.y += 1.;
        self.generate_ray(&sample)
      };
      // This is used for anti-aliasing, for example

      // Make sure to use the weight from the first ray, but if either of our differentials were 0, use that
      // TODO: Not sure why the book does this
      let wt = if wtx == 0. || wty == 0. { 0. } else { wt };
      (wt, RayDifferential { ray, ray_x, ray_y })
    }
}

#[enum_dispatch(Camera)]
pub enum CameraInstance {
  PerspectiveCamera,
}

pub struct PerspectiveCamera {
  pub film: Arc<Film>,
  pub position: Point3<f32>,
}

impl Camera for PerspectiveCamera {
  fn bounds(&self) -> Bounds2<i32> {
    self.film.bounds()
  }
  fn film(&self) -> Arc<Film> {
    self.film.clone()
  }
  fn generate_ray(&self, sample: &CameraSample) -> (f32, Ray) {
    let direction = Vector3 { x: (sample.film_point.x - 50.) / 100., y: (sample.film_point.y - 50.) / 100., z: 1. }.normalized();
    // TODO: perspective
    (1., Ray {
      origin: self.position,
      direction,
    })
  }
}