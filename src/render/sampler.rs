use enum_dispatch::enum_dispatch;

use crate::geometry::Point2;

#[enum_dispatch]
pub trait Sampler {
  fn start_pixel(&mut self, p: &Point2<i32>);
  fn start_next(&mut self) -> bool;
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
  fn start_pixel(&mut self, _: &Point2<i32>) {}
  fn start_next(&mut self) -> bool { false }
}

#[derive(Clone)]
pub struct RandomSampler {
  // TODO: pre-sample for performance
  pub samples_per_pixel: i64,
  pub current_sample: i64,
}

impl Sampler for RandomSampler {
  fn start_pixel(&mut self, _: &Point2<i32>) {
    self.current_sample = 0;
  }
  fn start_next(&mut self) -> bool {
    self.current_sample += 1;
    self.current_sample < self.samples_per_pixel
  }
}