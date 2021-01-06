use super::{Point2, Point3};

#[derive(Default, Copy, Clone, Debug)]
pub struct Bounds3<T> {
  min: Point3<T>,
  max: Point3<T>,
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