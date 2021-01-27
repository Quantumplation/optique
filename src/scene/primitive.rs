
use enum_dispatch::enum_dispatch;
use crate::{geometry::{Bounds3, Ray, Interaction}};

use super::{AreaLight, MaterialInstance, Shape, ShapeInstance};
#[enum_dispatch]
pub trait Primitive {
  fn bounds(&self) -> Bounds3<f64>;
  fn intersect(&self, ray: &Ray) -> Option<Interaction>;
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
  fn intersect(&self, _ray: &Ray) -> Option<Interaction> {
    None
  }
}

pub struct GeometricPrimitive {
  pub shape: ShapeInstance,
  pub material: Option<MaterialInstance>,
  pub emission: Option<AreaLight>,
}


impl Primitive for GeometricPrimitive {
  fn bounds(&self) -> Bounds3<f64> {
    self.shape.bounds()
  }

  fn intersect(&self, ray: &Ray) -> Option<Interaction> {
    self.shape.intersect(ray).map(|intersection| {
      Interaction {
        intersection,
        emission: self.emission.clone(),
        material: self.material.clone(),
      }
    }) 
  }
}

pub struct PrimitiveList {
  pub primitives: Vec<PrimitiveInstance>,
}

impl Primitive for PrimitiveList {
    fn bounds(&self) -> Bounds3<f64> {
      Bounds3::default()
    }

    fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        let mut min_dist = -1.;
        let mut min_interaction = None;
        for p in &self.primitives {
          if let Some(i) = p.intersect(ray) {
            if min_dist < 0. || i.intersection.distance < min_dist {
              min_dist = i.intersection.distance;
              min_interaction = Some(i);
            }
          }
        }
        min_interaction
    }
}