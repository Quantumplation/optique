use enum_dispatch::enum_dispatch;

use crate::geometry::{Bounds3, ErrorFloat, InteractionCommon, MulWithError, Ray, SurfaceInteraction, Transform, Vector3};

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
    let (ray, err) = self.object_to_world.inverse().mul_with_error(*ray);

    // Track error in the origin
    let (ox, oy, oz) = (
      ErrorFloat::new(ray.origin.x, err.origin.x),
      ErrorFloat::new(ray.origin.y, err.origin.y),
      ErrorFloat::new(ray.origin.z, err.origin.z),
    );
    let (dx, dy, dz) = (
      ErrorFloat::new(ray.direction.x, err.direction.x),
      ErrorFloat::new(ray.direction.y, err.direction.y),
      ErrorFloat::new(ray.direction.z, err.direction.z),
    );

    let radius: ErrorFloat = self.radius.into();
    let a = dx * dx + dy * dy + dz * dz;
    let b = 2. * (dx * ox + dy * oy + dz * oz);
    let c = ox * ox + oy * oy + oz * oz - radius * radius;

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

    let point_hit: Vector3<_> = Vector3::from(ray.origin + ray.direction * t_collision.value);
    let mut point_hit: Vector3<_> = point_hit * (self.radius / Vector3::from(point_hit).length());
    // Avoid 0 vectors
    if point_hit.x == 0. && point_hit.y == 0. {
      point_hit.x = 1e-5 * self.radius;
    }

    let two_pi = 2. * 3.1415926535;
    let phi = point_hit.y.atan2(point_hit.x);
    let phi = if phi < 0. { phi + two_pi } else { phi };

    let u = phi / two_pi;
    let theta = (point_hit.z / self.radius).clamp(-1., 1.).acos();
    let v = theta / two_pi;

    // TODO: better normals?
    let pos = self.object_to_world * Vector3::default();
    let normal = (point_hit - pos).normalized();

    Some(SurfaceInteraction {
      common: InteractionCommon {
        point: point_hit.into(),
        reverse_ray: -ray.direction,
        normal,
        intersection_time: t_collision.value,
      },
      emissive_properties: None,
    })
  }
}