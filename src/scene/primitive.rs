use crate::geometry::Bounds3f;
#[derive(Clone)]
pub enum Primitive {
  Null,
}

impl Primitive {
  pub fn bounds(&self) -> Bounds3f {
    Bounds3f {}
  }
}