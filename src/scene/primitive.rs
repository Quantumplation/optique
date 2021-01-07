use enum_dispatch::enum_dispatch;
use crate::geometry::{Bounds3, Ray, SurfaceInteraction};

use super::{Shape, ShapeInstance};
#[enum_dispatch]
pub trait Primitive {
  fn bounds(&self) -> Bounds3<f32>;
  fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction>;
}

#[enum_dispatch(Primitive)]
pub enum PrimitiveInstance {
  NullPrimitive,
  GeometricPrimitive,
}

pub struct NullPrimitive {}

impl Primitive for NullPrimitive {
  fn bounds(&self) -> Bounds3<f32> {
    Bounds3::default()
  }

  fn intersect(&self, _ray: &Ray) -> Option<SurfaceInteraction> {
    None
  }
}

pub struct GeometricPrimitive {
  pub shape: ShapeInstance,
}

impl Primitive for GeometricPrimitive {
  fn bounds(&self) -> Bounds3<f32> {
    self.shape.bounds()
  }

  fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
    self.shape.intersect(ray)
  }
}