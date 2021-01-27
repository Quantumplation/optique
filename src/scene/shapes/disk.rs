use std::f64::consts;

use crate::{geometry::{Bounds3, Intersection, MulWithError, Normal3, Point3, Transform, Vector3}, scene::Shape};

pub struct DiskShape {
  pub object_to_world: Transform,
  pub height: f64,
  pub radius: f64,
  pub inner_radius: f64,
}

impl Shape for DiskShape {
  fn bounds(&self) -> crate::geometry::Bounds3 {
    Bounds3 {
      min: Point3 { x: -self.radius, y: -self.radius, z: self.height },
      max: Point3 { x: self.radius, y: self.radius, z: self.height },
    }
  }

  fn intersect(&self, ray: &crate::geometry::Ray) -> Option<crate::geometry::Intersection> {
    let world_to_object = &self.object_to_world.inverse();
    let (ray, err) = world_to_object.mul_with_error(*ray);
    
    // The disk is 2D, so if the ray is parallel to this, bail early
    if ray.direction.z == 0. {
      return None;
    }


    // Find the time it intersects with the plane of the disk
    let hit_time = (self.height - ray.origin.z) / ray.direction.z;
    if hit_time <= 0. || hit_time >= ray.time_max {
      return None;
    }

    let hit_point: Point3 = ray.origin + ray.direction * hit_time;
    let hit_radius_sq = hit_point.x * hit_point.x + hit_point.y * hit_point.y;
    if hit_radius_sq > self.radius * self.radius || hit_radius_sq < self.inner_radius * self.inner_radius {
      return None;
    }

    // Find the parametric coordinates
    let phi = hit_point.y.atan2(hit_point.x);
    let phi = if phi < 0. { phi + consts::TAU } else { phi };
    let u = phi / consts::TAU;
    let hit_radius = hit_radius_sq.sqrt();
    let v = 1. - (hit_radius - self.inner_radius) / (self.radius - self.inner_radius);

    let dpdu = Vector3::new(-consts::TAU * hit_point.y, consts::TAU * hit_point.x, 0.);
    let dpdv: Vector3 = Vector3::new(hit_point.x, hit_point.y, 0.) * (self.inner_radius - self.radius) / hit_radius;
    let normal: Normal3 = dpdu.cross(dpdv).normalized().into();
    let dndu = Normal3::new(0., 0., 0.);
    let dndv = Normal3::new(0., 0., 0.);
    
    // Refine the intersection point
    let hit_point = Point3::new(hit_point.x, hit_point.y, self.height);

    Some(self.object_to_world * Intersection {
      point: hit_point,
      point_derivative: (dpdu, dpdv),
      error: Vector3::default(),
      distance: hit_time,
      outgoing: -ray.direction,
      normal,
      normal_derivative: (dndu, dndv),
      shading_normal: normal,
      shading_normal_derivative: (dndu, dndv),
    })
  }
}