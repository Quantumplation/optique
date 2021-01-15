use std::ops::{Add, Mul, Sub};

use super::{Vector2, Vector3};

#[derive(Default, Copy, Clone, Debug)]
pub struct Point2<T> {
  pub x: T,
  pub y: T,
}

impl<T> Point2<T> {
  pub fn new(x: T, y: T) -> Self {
    Point2 { x, y }
  }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Point3<T> {
  pub x: T,
  pub y: T,
  pub z: T
}

impl<T> Point3<T> {
  pub fn new(x: T, y: T, z: T) -> Self {
    Point3 { x, y, z }
  }
}

impl From<Point2<i32>> for Point2<f32> {
  fn from(p: Point2<i32>) -> Self {
    Self { x: p.x as f32, y: p.y as f32 }
  }
}

impl<T: Add<Output = T>> Add for Point2<T> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
  }
}
impl<T: Add<Output = T>> Add<Vector2<T>> for Point2<T> {
  type Output = Point2<T>;

  fn add(self, rhs: Vector2<T>) -> Self::Output {
    Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
  }
}
impl<T: Add<Output = T>> Add<Vector3<T>> for Point3<T> {
  type Output = Point3<T>;

  fn add(self, rhs: Vector3<T>) -> Self::Output {
    Self::Output { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
  }
}
impl<T: Add<Output = T>> Add for Point3<T> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self::Output { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
  }
}

impl<T: Sub<Output = T>> Sub for Point2<T> {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self::Output { x: self.x - rhs.x, y: self.y - rhs.y }
  }
}
impl<T: Sub<Output = T>> Sub for Point3<T> {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self::Output { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
  }
}

impl<T: Mul<T, Output = T> + Copy> Mul<T> for Point2<T> {
  type Output = Self;
  fn mul(self, s: T) -> Self::Output {
    Self::Output { x: s * self.x, y: s * self.y }
  }
}
impl<T: Mul<T, Output = T> + Copy> Mul<T> for Point3<T> {
  type Output = Self;
  fn mul(self, s: T) -> Self::Output {
    Self::Output { x: s * self.x, y: s * self.y, z: s * self.z }
  }
}
