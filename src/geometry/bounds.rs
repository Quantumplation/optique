use std::cmp::min;

use super::{Point2, Point3, Ray, Vector3, gamma};

#[derive(Default, Copy, Clone, Debug)]
pub struct Bounds3<T = f64> {
  pub min: Point3<T>,
  pub max: Point3<T>,
}

impl<T> Bounds3<T> {
  pub fn new(min: Point3<T>, max: Point3<T>) -> Self {
    Bounds3 { min, max }
  }
}

impl Bounds3 {
  pub fn union(&self, other: &Self) -> Self {
    Self {
      min: self.min.min(&other.min),
      max: self.max.max(&other.max),
    }
  }
  pub fn encompass(&self, other: Point3) -> Self {
    Self {
      min: self.min.min(&other),
      max: self.max.max(&other),
    }
  }
  
  pub fn maximum_dimension(&self) -> u8 {
    let range = self.max - self.min;
    if range.x > range.y {
      if range.x > range.z {
        0
      } else {
        2
      }
    } else if range.y > range.z {
      1
    } else {
      2
    }
  }
  
  pub fn any_intersect_precomputed(&self, ray: &Ray, inv_dir: Vector3, is_neg: [usize; 3]) -> bool {
    let bounds = [self.min, self.max];
    // Check for collisions with the x and y slabs
    let tx_min = (bounds[is_neg[0]].x - ray.origin.x) * inv_dir.x;
    let tx_max = (bounds[1 - is_neg[0]].x - ray.origin.x) * inv_dir.x;
    let ty_min = (bounds[is_neg[1]].y - ray.origin.y) * inv_dir.y;
    let ty_max = (bounds[1 - is_neg[1]].y - ray.origin.y) * inv_dir.y;

    // Update to account for floating point error
    let tx_max = tx_max * (1. + 2. * gamma(3));
    let ty_max = ty_max * (1. + 2. * gamma(3));

    if tx_min > ty_max || ty_min > tx_max { return false; }
    let t_min = tx_min.max(ty_min);
    let t_max = tx_max.min(ty_max);

    // Check against the z slabs
    let tz_min = (bounds[is_neg[2]].z - ray.origin.z) * inv_dir.z;
    let tz_max = (bounds[1 - is_neg[2]].z - ray.origin.z) * inv_dir.z;

    let tz_max = tz_max * (1. + 2. * gamma(3));

    if t_min > tz_max || tz_min > t_max { return false; }

    let t_min = t_min.max(tz_min);
    let t_max = t_max.min(tz_max);

    return t_min < ray.time_max && t_max > 0.;
  }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Bounds2<T> {
  pub min: Point2<T>,
  pub max: Point2<T>,
}
  
pub struct PixelIterator {
  pub bounds: Bounds2<i32>,
  pub curr: Point2<i32>,
}
  
impl Iterator for PixelIterator {
  type Item = Point2<i32>;

  fn next(&mut self) -> Option<Self::Item> {
    self.curr.x += 1;
    if self.curr.x == self.bounds.max.x {
      self.curr.x = self.bounds.min.x;
      self.curr.y += 1;
    }
    if self.curr.y >= self.bounds.max.y {
      None
    } else {
      Some(self.curr)
    }
  }
}

impl IntoIterator for Bounds2<i32> {
  type Item = Point2<i32>;
  type IntoIter = PixelIterator;

  fn into_iter(self) -> Self::IntoIter {
    // Return an iterator starting 1 before the start of our bounds
    // because we increment first thing in next
    PixelIterator {
      bounds: self,
      curr: Point2 { x: self.min.x - 1, y: self.min.y }
    }
  }
}