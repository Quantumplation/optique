use enum_dispatch::enum_dispatch;
use crate::geometry::Bounds3f;
#[enum_dispatch]
pub trait Primitive {
  fn bounds(&self) -> Bounds3f;
}

#[enum_dispatch(Primitive)]
#[derive(Clone)]
pub enum PrimitiveInstance {
  NullPrimitive
}

#[derive(Clone)]
pub struct NullPrimitive {}

impl Primitive for NullPrimitive {
  fn bounds(&self) -> Bounds3f {
    Bounds3f {}
  }
}