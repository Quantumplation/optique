use std::ops::{Add, Mul, Sub};

use super::{Point2, Point3};

#[derive(Default, Copy, Clone, Debug)]
pub struct Vector2<T> {
  pub x: T,
  pub y: T,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Vector3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl Vector3<f32> {
  pub fn length_squared(&self) -> f32 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }
  pub fn length(&self) -> f32 {
    self.length_squared().sqrt()
  }
  pub fn normalized(&self) -> Vector3<f32> {
    let len = self.length();
    Vector3 { x: self.x / len, y: self.y / len, z: self.z / len }
  }
  pub fn dot(&self, rhs: Vector3<f32>) -> f32 {
    self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
  }
}

impl<T> From<Point2<T>> for Vector2<T> {
  fn from(p: Point2<T>) -> Self {
    Vector2 { x: p.x, y: p.y }
  }
}
impl<T> From<Point3<T>> for Vector3<T> {
  fn from(p: Point3<T>) -> Self {
    Vector3 { x: p.x, y: p.y, z: p.z }
  }
}

impl<T: Add<Output = T>> Add for Vector2<T> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self::Output { x: self.x + rhs.x, y: self.y + rhs.y }
  }
}
impl<T: Add<Output = T>> Add for Vector3<T> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self::Output { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
  }
}

impl<T: Sub<Output = T>> Sub for Vector2<T> {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self::Output { x: self.x - rhs.x, y: self.y - rhs.y }
  }
}
impl<T: Sub<Output = T>> Sub for Vector3<T> {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self::Output { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
  }
}

impl<T: Mul<T, Output = T> + Copy> Mul<T> for Vector2<T> {
  type Output = Self;
  fn mul(self, s: T) -> Self::Output {
    Self::Output { x: s * self.x, y: s * self.y }
  }
}
impl<T: Mul<T, Output = T> + Copy> Mul<T> for Vector3<T> {
  type Output = Self;
  fn mul(self, s: T) -> Self::Output {
    Self::Output { x: s * self.x, y: s * self.y, z: s * self.z }
  }
}