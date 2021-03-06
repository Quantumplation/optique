use std::ops::{Add, Div, Index, Mul, Sub};

use crate::utils::{next_down, next_up};

use super::{Normal3, Vector2, Vector3};

#[derive(Default, Copy, Clone, Debug)]
pub struct Point2<T = f64> {
  pub x: T,
  pub y: T,
}

impl<T> Point2<T> {
  #[allow(dead_code)]
  pub fn new(x: T, y: T) -> Self {
    Point2 { x, y }
  }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Point3<T = f64> {
  pub x: T,
  pub y: T,
  pub z: T
}

impl<T> Point3<T> {
  pub fn new(x: T, y: T, z: T) -> Self {
    Point3 { x, y, z }
  }
}
  
impl<T> Index<u8> for Point3<T> {
  type Output = T;

  fn index(&self, index: u8) -> &Self::Output {
    match index {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => panic!("Invalid dimension"),
    }
  }
}

impl Point3 {
  pub fn min(&self, other: &Self) -> Self {
    Point3 { x: self.x.min(other.x), y: self.y.min(other.y), z: self.z.min(other.z) }
  }
  pub fn max(&self, other: &Self) -> Self {
    Point3 { x: self.x.max(other.x), y: self.y.max(other.y), z: self.z.max(other.z) }
  }
}

impl Point3 {
  pub fn offset_for_error(&self, error: Vector3, normal: Normal3, reverse: Vector3) -> Self {
    let distance = normal.abs().dot(error.into());
    let offset: Vector3 = if reverse.dot(normal.into()) < 0. {
      -normal * distance
    } else {
      normal * distance
    }.into();

    let mut origin = *self + offset;
    // Round the offset point away from p
    if offset.x > 0. { origin.x = next_up(origin.x); }
    else if offset.x < 0. { origin.x = next_down(origin.x); }

    if offset.y > 0. { origin.y = next_up(origin.y); }
    else if offset.y < 0. { origin.y = next_down(origin.y); }

    if offset.z > 0. { origin.z = next_up(origin.z); }
    else if offset.z < 0. { origin.z = next_down(origin.z); }
    
    origin
  }
}

impl From<Point2<u32>> for Point2 {
  fn from(p: Point2<u32>) -> Self {
    Self { x: p.x as f64, y: p.y as f64 }
  }
}

impl<T, U> From<Vector3<U>> for Point3<T>
  where T: From<U> {
  fn from(v: Vector3<U>) -> Self {
    Point3 { x: v.x.into(), y: v.y.into(), z: v.z.into() }
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
  type Output = Vector2<T>;

  fn sub(self, rhs: Self) -> Self::Output {
    Self::Output { x: self.x - rhs.x, y: self.y - rhs.y }
  }
}
impl<T: Sub<Output = T>> Sub for Point3<T> {
  type Output = Vector3<T>;

  fn sub(self, rhs: Self) -> Self::Output {
    Self::Output { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
  }
}
impl<T: Sub<Output = T>> Sub<Vector3<T>> for Point3<T> {
  type Output = Point3<T>;

  fn sub(self, rhs: Vector3<T>) -> Self::Output {
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

impl<T: Div<T, Output = T> + Copy> Div<T> for Point3<T> {
  type Output = Self;

  fn div(self, s: T) -> Self::Output {
    Self { x: self.x / s, y: self.y / s, z: self.z / s }
  }
}