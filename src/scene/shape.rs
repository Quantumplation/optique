use enum_dispatch::enum_dispatch;

use crate::geometry::{Bounds3, Intersection, Ray, Transform};

use super::{SphereShape, DiskShape, TriangleShape};

#[enum_dispatch]
pub trait Shape {
  fn object_to_world(&self) -> Transform;
  fn bounds(&self) -> Bounds3<f64>;
  fn world_bounds(&self) -> Bounds3<f64> {
    self.object_to_world() * self.bounds()
  }
  fn intersect(&self, ray: &Ray) -> Option<Intersection>;
  fn any_intersect(&self, ray: &Ray) -> bool { self.intersect(ray).is_some() }
}
#[enum_dispatch(Shape)]
pub enum ShapeInstance {
  NullShape,
  SphereShape,
  DiskShape,
  TriangleShape,
}

pub struct NullShape {}

impl Shape for NullShape {
  fn object_to_world(&self) -> Transform { Transform::default() }
  fn bounds(&self) -> Bounds3<f64> { Bounds3::default() }
  fn intersect(&self, _ray: &Ray) -> Option<Intersection> { None }
}