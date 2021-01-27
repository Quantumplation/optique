use enum_dispatch::enum_dispatch;

use crate::geometry::{Bounds3, Intersection, Ray};

use super::{SphereShape, DiskShape};

#[enum_dispatch]
pub trait Shape {
  fn bounds(&self) -> Bounds3<f64>;
  fn intersect(&self, ray: &Ray) -> Option<Intersection>;
  fn any_intersect(&self, ray: &Ray) -> bool { self.intersect(ray).is_some() }
}
#[enum_dispatch(Shape)]
pub enum ShapeInstance {
  NullShape,
  SphereShape,
  DiskShape,
}

pub struct NullShape {}

impl Shape for NullShape {
    fn bounds(&self) -> Bounds3<f64> { Bounds3::default() }
    fn intersect(&self, _ray: &Ray) -> Option<Intersection> { None }
}