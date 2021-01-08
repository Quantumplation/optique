use enum_dispatch::enum_dispatch;

use crate::geometry::{Bounds3, InteractionCommon, Point3, Ray, SurfaceInteraction, Vector3};

use super::PrimitiveInstance;

#[enum_dispatch]
pub trait Shape {
  fn bounds(&self) -> Bounds3<f32>;
  fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction>;
}
#[enum_dispatch(Shape)]
pub enum ShapeInstance {
  NullShape,
  SphereShape,
}

pub struct NullShape {}

impl Shape for NullShape {
    fn bounds(&self) -> Bounds3<f32> { Bounds3::default() }
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> { None }
}

pub struct SphereShape {
  pub point: Point3<f32>,
  pub radius: f32,
}

impl Shape for SphereShape {
  fn bounds(&self) -> Bounds3<f32> { Bounds3::default() }
  fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
    // Find the time halfway between the two intersections
    let offset = Vector3::<f32>::from(self.point - ray.origin);
    let t_center = offset.dot(ray.direction);
    // If this is negative, the ray is pointing in the wrong direction
    if t_center < 0. { return None; }

    let delta_sq = offset.length_squared() - t_center * t_center;
    let radius_sq = self.radius * self.radius;
    if delta_sq > radius_sq { return None; }

    let t_gap = (radius_sq - delta_sq).sqrt();

    let (t_0, t_1) = (t_center - t_gap, t_center + t_gap);
    let (t_0, t_1) = if t_0 > t_1 { (t_1, t_0) } else { (t_0, t_1) };

    if t_0 < 0. && t_1 < 0. {
      None
    } else {
      let point = ray.origin + (ray.direction * t_0);
      let reverse_ray = -ray.direction;
      let normal = Vector3::from(point - self.point);
      Some(
        SurfaceInteraction {
          common: InteractionCommon { 
            point,
            reverse_ray,
            normal,
          },
          emissive_properties: None,
        }
      )
    }
  }
}