use std::ops::{Add, Div, Index, Mul, Neg, Sub};

use super::{Normal3, Point2, Point3};

#[derive(Default, Copy, Clone, Debug)]
pub struct Vector2<T = f64> {
  pub x: T,
  pub y: T,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Vector3<T = f64> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl<T> Vector3<T> {
  pub fn new(x: T, y: T, z: T) -> Self {
    Vector3 { x, y, z }
  }
}

impl Vector3 {
  pub fn length_squared(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }
  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }
  pub fn normalized(&self) -> Vector3 {
    let len = self.length();
    Vector3 { x: self.x / len, y: self.y / len, z: self.z / len }
  }
  pub fn dot(&self, rhs: Vector3) -> f64 {
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
  pub fn abs(&self) -> Self {
    Self { x: self.x.abs(), y: self.y.abs(), z: self.z.abs() }
  }
  pub fn reflect(&self, normal: Normal3) -> Self {
    // NOTE: assumes the normal is normalized
    let dot = self.dot(normal.into());
    let offset: Vector3 = Into::<Vector3>::into(normal) * 2. * dot;
    return *self - offset;
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

impl<T> Index<u8> for Vector3<T> {
  type Output = T;

  fn index(&self, index: u8) -> &Self::Output {
    match index {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => panic!("Invalid axis"),
    }
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

impl<T: Neg<Output = T>> Neg for Vector3<T> {
    type Output = Vector3<T>;

    fn neg(self) -> Self::Output {
      Vector3 { x: -self.x, y: -self.y, z: -self.z }
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

impl<T: Div<T, Output = T> + Copy> Div<T> for Vector2<T> {
  type Output = Self;
  fn div(self, s: T) -> Self::Output {
    Self::Output { x: self.x / s, y: self.y / s }
  }
}
impl<T: Div<T, Output = T> + Copy> Div<T> for Vector3<T> {
  type Output = Self;
  fn div(self, s: T) -> Self::Output {
    Self::Output { x: self.x / s, y: self.y / s, z: self.z / s }
  }
}