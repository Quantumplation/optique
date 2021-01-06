use crate::geometry::{Bounds2, Point2};

pub struct Film {
  pub resolution: Point2<i32>,
}

impl Film {
  pub fn bounds(&self) -> Bounds2<i32> {
    Bounds2 { min: Default::default(), max: self.resolution }
  }
}