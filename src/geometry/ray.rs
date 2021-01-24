use super::{Point3, Vector3};

#[derive(Clone, Copy)]
pub struct Ray {
  pub origin: Point3,
  pub direction: Vector3,
  pub time_max: f64,
}

impl Ray {
  pub fn reflect(&self, point: Point3, normal: Vector3) -> Self {
    Ray {
      origin: point,
      direction: self.direction.reflect(normal),
      time_max: self.time_max,
    }
  }
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

  pub fn reflect(&self, point: Point3, normal: Vector3) -> Self {
    RayDifferential {
      ray: self.ray.reflect(point, normal),
      ray_x: self.ray_x.reflect(point, normal),
      ray_y: self.ray_y.reflect(point, normal),
    }
  }
}