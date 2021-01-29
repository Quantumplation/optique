use enum_dispatch::enum_dispatch;

use crate::geometry::Point2;

#[enum_dispatch]
pub trait Sampler {
  fn start_pixel(&mut self, p: &Point2<u32>);
  fn start_next(&mut self) -> bool;
  
  fn samples_per_pixel(&self) -> i64;

  fn get_camera_sample(&self, raster_point: Point2<u32>) -> CameraSample {
    CameraSample {
      film_point: Point2::<f64>::from(raster_point) + Point2::default(),
      lens_point: Point2::default(),
    }
  }
}

#[derive(Clone)]
pub struct CameraSample {
  pub film_point: Point2,
  pub lens_point: Point2,
}

#[enum_dispatch(Sampler)]
#[derive(Clone)]
pub enum SamplerInstance {
  NullSampler,
  RandomSampler
}

#[derive(Clone)]
pub struct NullSampler {}

impl Sampler for NullSampler {
  fn start_pixel(&mut self, _: &Point2<u32>) {}
  fn start_next(&mut self) -> bool { false }
  fn samples_per_pixel(&self) -> i64 { 0 }
}

#[derive(Clone)]
pub struct RandomSampler {
  // TODO: pre-sample for performance
  pub samples_per_pixel: i64,
  pub current_sample: i64,
}

impl Sampler for RandomSampler {
  fn start_pixel(&mut self, _: &Point2<u32>) {
    self.current_sample = 0;
  }
  fn start_next(&mut self) -> bool {
    self.current_sample += 1;
    self.current_sample < self.samples_per_pixel
  }
  fn samples_per_pixel(&self) -> i64 { self.samples_per_pixel }
}