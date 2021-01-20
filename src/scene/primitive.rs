
use enum_dispatch::enum_dispatch;
use crate::geometry::{Bounds3, Ray, SurfaceInteraction};

use super::{AreaLight, Shape, ShapeInstance};
#[enum_dispatch]
pub trait Primitive {
  fn bounds(&self) -> Bounds3<f64>;
  fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction>;
  fn emissive_properties(&self) -> Option<AreaLight>;
}

#[enum_dispatch(Primitive)]
pub enum PrimitiveInstance {
  NullPrimitive,
  GeometricPrimitive,
  PrimitiveList,
}

pub struct NullPrimitive {}

impl Primitive for NullPrimitive {
  fn bounds(&self) -> Bounds3<f64> {
    Bounds3::default()
  }

  fn intersect(&self, _ray: &Ray) -> Option<SurfaceInteraction> {
    None
  }
  fn emissive_properties(&self) -> Option<AreaLight> {
    None
  }
}

pub struct GeometricPrimitive {
  pub shape: ShapeInstance,
  pub emission: Option<AreaLight>,
}

impl Primitive for GeometricPrimitive {
  fn bounds(&self) -> Bounds3<f64> {
    self.shape.bounds()
  }

  fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
    self.shape.intersect(ray).map(|interaction| {
      SurfaceInteraction {
        emissive_properties: self.emissive_properties(),
        ..interaction
      }
    }) 
  }

  fn emissive_properties(&self) -> Option<AreaLight> {
    self.emission.clone()
  }
}

pub struct PrimitiveList {
  pub primitives: Vec<PrimitiveInstance>,
}

impl Primitive for PrimitiveList {
    fn bounds(&self) -> Bounds3<f64> {
      Bounds3::default()
    }

    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        let mut min_dist = -1.;
        let mut min_interaction = None;
        for p in &self.primitives {
          if let Some(i) = p.intersect(ray) {
            if min_dist < 0. || i.common.intersection_time < min_dist {
              min_dist = i.common.intersection_time;
              min_interaction = Some(i);
            }
          }
        }
        min_interaction
    }

    fn emissive_properties(&self) -> Option<AreaLight> {
      None
    }
}