use std::ops::{Add, Mul, Neg};

use super::Vector3;

#[derive(Copy, Clone, Default)]
pub struct Normal3<T = f64> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl<T> Normal3<T> {
  pub fn new(x: T, y: T, z: T) -> Self {
    Self { x, y, z }
  }
}

impl Normal3 {
  pub fn abs(&self) -> Self {
    Self { x: self.x.abs(), y: self.y.abs(), z: self.z.abs() }
  }

  pub fn dot(&self, rhs: Self) -> f64 {
    self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
  }

  pub fn cross(&self, rhs: Self) -> Self {
    let (x,y,z) = (self.x, self.y, self.z);
    let (ox, oy, oz) = (rhs.x, rhs.y, rhs.z);
    Self {
      x: (y * oz) - (z * oy),
      y: (z * ox) - (x * oz),
      z: (x * oy) - (y * ox)
    }
  }

  pub fn length_squared(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn normalized(&self) -> Normal3 {
    let len = self.length();
    Normal3 { x: self.x / len, y: self.y / len, z: self.z / len }
  }
}

impl<T> From<Vector3<T>> for Normal3<T> {
  fn from(other: Vector3<T>) -> Self {
    Self { x: other.x, y: other.y, z: other.z }
  }
}
impl<T> Into<Vector3<T>> for Normal3<T> {
  fn into(self) -> Vector3<T> {
    Vector3 { x: self.x, y: self.y, z: self.z }
  }
}

impl<T: Neg<Output = T>> Neg for Normal3<T> {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self::Output { x: -self.x, y: -self.y, z: -self.z }
  }
}

impl<T: Add<Output = T>> Add for Normal3<T> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self::Output { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
  }
}

impl<T: Mul<f64, Output = T>> Mul<f64> for Normal3<T> {
  type Output = Self;

  fn mul(self, s: f64) -> Self::Output {
    Self::Output{ x: self.x * s, y: self.y * s, z: self.z * s }
  }
}