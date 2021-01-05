use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Shape {}
#[enum_dispatch(Shape)]
pub enum ShapeInstance {
  NullShape,
}

pub struct NullShape {}

impl Shape for NullShape {}