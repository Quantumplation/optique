use enum_dispatch::enum_dispatch;
use crate::geometry::Bounds3;
#[enum_dispatch]
pub trait Primitive {
  fn bounds(&self) -> Bounds3<f32>;
}

#[enum_dispatch(Primitive)]
#[derive(Clone)]
pub enum PrimitiveInstance {
  NullPrimitive
}

#[derive(Clone)]
pub struct NullPrimitive {}

impl Primitive for NullPrimitive {
  fn bounds(&self) -> Bounds3<f32> {
    Bounds3::default()
  }
}