use enum_dispatch::enum_dispatch;

use crate::geometry::{Bounds3, ErrorFloat, InteractionCommon, MulWithError, Point3, Ray, SurfaceInteraction, Transform, Vector3, gamma};

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
      point_hit.x = 1e-5 * self.radius;
    }

    let two_pi = 2. * 3.1415926535;
    let phi = point_hit.y.atan2(point_hit.x);
    let phi = if phi < 0. { phi + two_pi } else { phi };

    let _u = phi / two_pi;
    let theta = (point_hit.z / self.radius).clamp(-1., 1.).acos();
    let _v = theta / two_pi;

    // TODO: better normals?
    let pos = self.object_to_world * Point3::default();
    let normal = Vector3::from(point_hit - pos).normalized();
    
    let error = Vector3::from(point_hit).abs() * gamma(5);

    Some(SurfaceInteraction {
      common: InteractionCommon {
        point: self.object_to_world * point_hit, // Make sure to translate the point back to world coordinates
        reverse_ray: -ray.direction,
        normal,
        intersection_time: t_collision.value,
        error,
      },
      emissive_properties: None,
    })
  }
}