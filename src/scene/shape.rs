use enum_dispatch::enum_dispatch;

use crate::geometry::{Bounds3, ErrorFloat, InteractionCommon, MulWithError, Normal3, PI, Point3, Ray, SurfaceInteraction, Transform, Vector3, gamma};

#[enum_dispatch]
pub trait Shape {
  fn bounds(&self) -> Bounds3<f64>;
  fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction>;
  fn any_intersect(&self, ray: &Ray) -> bool { self.intersect(ray).is_some() }
}
#[enum_dispatch(Shape)]
pub enum ShapeInstance {
  NullShape,
  SphereShape,
}

pub struct NullShape {}

impl Shape for NullShape {
    fn bounds(&self) -> Bounds3<f64> { Bounds3::default() }
    fn intersect(&self, _ray: &Ray) -> Option<SurfaceInteraction> { None }
}

pub struct SphereShape {
  pub object_to_world: Transform,
  pub radius: f64,
}

impl Shape for SphereShape {
  fn bounds(&self) -> Bounds3<f64> { Bounds3::default() }
  fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
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

    let two_pi = 2. * PI;
    let max_phi = two_pi;
    let max_theta = two_pi;
    let phi = point_hit.y.atan2(point_hit.x);
    let phi = if phi < 0. { phi + two_pi } else { phi };

    let u = phi / max_phi;
    let theta = (point_hit.z / self.radius).clamp(-1., 1.).acos();
    let v = theta / max_theta;

    let z_radius = Vector3::from(point_hit).length();
    let inv_radius = 1. / z_radius;
    let cos_phi = point_hit.x * inv_radius;
    let sin_phi = point_hit.y * inv_radius;

    // Compute point partial derivatives
    let dpdu = Vector3::new(-max_phi * point_hit.y, max_phi * point_hit.x, 0.);
    let dpdv: Vector3 = Vector3::new(
      point_hit.z * cos_phi,
      point_hit.z * sin_phi,
      -radius.value * theta.sin()
    ) * max_phi;

    // Compute second order partial derivatives
    let d2pduu = Vector3::new(point_hit.x, point_hit.y, 0.) * -max_phi * max_phi;
    let d2pduv = Vector3::new(-sin_phi, cos_phi, 0.) * max_theta * point_hit.z * max_phi;
    let d2pdvv = Vector3::new(point_hit.x, point_hit.y, point_hit.z) * -max_theta * max_theta;

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

    Some(SurfaceInteraction {
      common: InteractionCommon {
        point: self.object_to_world * point_hit, // Make sure to translate the point back to world coordinates
        point_derivative: (dpdu, dpdv),
        reverse_ray: -ray.direction,
        normal,
        normal_derivative: (dndu, dndv),
        shading_normal: normal,
        shading_normal_derivative: (dndu, dndv),
        intersection_time: t_collision.value,
        error,
      },
      emissive_properties: None,
    })
  }
}