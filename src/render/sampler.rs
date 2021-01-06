use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Sampler {
}

#[enum_dispatch(Sampler)]
pub enum SamplerInstance {
  NullSampler,
}

pub struct NullSampler {}

impl Sampler for NullSampler {}