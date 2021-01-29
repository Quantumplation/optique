use std::f64::consts;

use crate::{geometry::{Bounds3, ErrorFloat, Intersection, MulWithError, Normal3, Point3, Ray, Transform, Vector3, gamma}, scene::Shape};


pub struct SphereShape {
  pub object_to_world: Transform,
  pub radius: f64,
}

impl Shape for SphereShape {
  fn object_to_world(&self) -> Transform {
    self.object_to_world
  }
  fn bounds(&self) -> Bounds3<f64> {
    Bounds3::new(
      Point3::new(-self.radius, -self.radius, -self.radius),
      Point3::new(self.radius, self.radius, self.radius),
    )
  }
  fn intersect(&self, ray: &Ray) -> Option<Intersection> {
    let world_to_object = self.object_to_world.inverse();
    let (ray, err) = world_to_object.mul_with_error(*ray);

    // Track error in the origin
    let (ox, oy, oz) = (
      ErrorFloat::new(ray.origin.x, err.0.x),
      ErrorFloat::new(ray.origin.y, err.0.y),
      ErrorFloat::new(ray.origin.z, err.0.z),
    );
    let (dx, dy, dz) = (
      ErrorFloat::new(ray.direction.x, err.1.x),
      ErrorFloat::new(ray.direction.y, err.1.y),
      ErrorFloat::new(ray.direction.z, err.1.z),
    );

    let radius: ErrorFloat = self.radius.into();
    let a: ErrorFloat = dx * dx + dy * dy + dz * dz;
    let b: ErrorFloat = 2. * (dx * ox + dy * oy + dz * oz);
    let c: ErrorFloat = ox * ox + oy * oy + oz * oz - (radius * radius);

    let quadratic = ErrorFloat::qudratic(a, b, c);
    if quadratic.is_none() {
      return None;
    }

    let (t0, t1) = quadratic.unwrap();
    if t0.high > ray.time_max || t1.low <= 0. {
      return None;
    }

    let t_collision = if t0.low <= 0. {
      t1
    } else {
      t0
    };

    if t_collision.high > ray.time_max {
      return None;
    }

    let point_hit: Point3<_> = ray.origin + ray.direction * t_collision.value;
    let mut point_hit: Point3<_> = point_hit * (self.radius / Vector3::from(point_hit).length());
    // Avoid 0 vectors
    if point_hit.x == 0. && point_hit.y == 0. {
      point_hit.x = 1e-10 * self.radius;
    }

    let max_phi = consts::TAU;
    let min_theta = consts::PI;
    let max_theta = 0.;
    let phi = point_hit.y.atan2(point_hit.x);
    let phi = if phi < 0. { phi + consts::TAU } else { phi };

    let u = phi / max_phi;
    let theta = (point_hit.z / self.radius).clamp(-1., 1.).acos();
    let v = (theta - min_theta) / (max_theta - min_theta);

    // NOTE: compute for a given theta, what radius circle does a cross section of the circle make?
    let z_radius = (point_hit.x * point_hit.x + point_hit.y * point_hit.y).sqrt();
    let inv_radius = 1. / z_radius;
    let cos_phi = point_hit.x * inv_radius;
    let sin_phi = point_hit.y * inv_radius;

    // Compute point partial derivatives
    let dpdu = Vector3::new(-max_phi * point_hit.y, max_phi * point_hit.x, 0.);
    let dpdv: Vector3 = Vector3::new(
      point_hit.z * cos_phi,
      point_hit.z * sin_phi,
      -radius.value * theta.sin()
    ) * (max_theta - min_theta);

    // Compute second order partial derivatives
    let d2pduu = Vector3::new(point_hit.x, point_hit.y, 0.) * -max_phi * max_phi;
    let d2pduv = Vector3::new(-sin_phi, cos_phi, 0.) * (max_theta - min_theta) * point_hit.z * max_phi;
    let d2pdvv = Vector3::new(point_hit.x, point_hit.y, point_hit.z) * -(max_theta - min_theta) * (max_theta - min_theta);

    // Compute coefficients of the fundamental form, to compute partial derivatives of the normal
    let E = dpdu.dot(dpdu);
    let F = dpdu.dot(dpdv);
    let G = dpdv.dot(dpdv);

    let normal = dpdu.cross(dpdv).normalized();

    let e = normal.dot(d2pduu);
    let f = normal.dot(d2pduv);
    let g = normal.dot(d2pdvv);

    // Compute the normal derivatives
    let inv_egf2 = 1. / (E * G - F * F);
    let dndu = Normal3::from(
      dpdu * (f * F - e * G) * inv_egf2 +
      dpdv * (e * F - f * E) * inv_egf2
    );
    let dndv = Normal3::from(
      dpdu * (g * F - f * G) * inv_egf2 +
      dpdv * (f * F - g * E) * inv_egf2
    );
    
    let normal = Normal3::from(normal);

    let error = Vector3::from(point_hit).abs() * gamma(5);

    Some(self.object_to_world * Intersection {
      point: point_hit,
      point_derivative: (dpdu, dpdv),
      outgoing: -ray.direction,
      normal,
      normal_derivative: (dndu, dndv),
      shading_normal: normal,
      shading_normal_derivative: (dndu, dndv),
      distance: t_collision.value,
      error,
    })
  }
}