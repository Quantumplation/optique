use super::{Point3, Vector3};

pub struct Ray {
  pub origin: Point3<f64>,
  pub direction: Vector3<f64>,
}

pub struct RayDifferential {
  pub ray: Ray,
  pub ray_x: Ray,
  pub ray_y: Ray,
}
  
impl RayDifferential {
  pub fn scale(&mut self, factor: f64) {
    let (origin, direction) = (self.ray.origin, self.ray.direction);
    self.ray_x.origin = origin + (self.ray_x.origin - origin) * factor;
    self.ray_x.direction = Vector3::from(origin) + (self.ray_x.direction - direction) * factor;
  }
}