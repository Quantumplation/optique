use std::{ops::{Add, Div, Mul, Neg, Sub}};

use float_next_after::NextAfter;

#[derive(Debug, Clone, Copy, Default)]
pub struct ErrorFloat {
  pub value: f64,
  pub low: f64,
  pub high: f64,
}

pub const UP: f64 = f64::INFINITY;
pub const DOWN: f64 = f64::NEG_INFINITY;

pub fn gamma(n: u32) -> f64 {
  let n = n as f64;
  (n * f64::EPSILON * 0.5) / (1. - n * f64::EPSILON * 0.5)
}

impl ErrorFloat {
  pub fn new(value: f64, error: f64) -> Self {
    if error == 0. {
      Self { value, low: value, high: value }
    } else {
      Self {
        value,
        low: (value - error).next_after(DOWN),
        high: (value + error).next_after(UP),
      }
    }
  }

  pub fn absolute_error(&self) -> f64 {
    let err_high = (self.high - self.value).abs();
    let err_low = (self.value - self.low).abs();
    err_high.max(err_low).next_after(UP)
  }

  pub fn sqrt(&self) -> Self {
    let value = self.value.sqrt();
    let low = self.low.sqrt().next_after(DOWN);
    let high = self.high.sqrt().next_after(UP);
    Self { value, low, high }
  }

  pub fn abs(&self) -> Self {
    if self.low >= 0. {
      // The entire interval is above zero, nothing to do
      self.clone()
    } else if self.high <= 0. {
      // The entire interval is below zero, we can just flip
      -self.clone()
    } else {
      // The interval straddles zero, so the abs of our low might be higher
      let value = self.value.abs();
      let low = 0.;
      let high = self.high.max(-self.low);
      Self { value, low, high }
    }
  }

  pub fn qudratic(a: ErrorFloat, b: ErrorFloat, c: ErrorFloat) -> Option<(ErrorFloat, ErrorFloat)> {
    let discriminant = b.value * b.value - 4. * a.value * c.value;
    if discriminant < 0. {
      return None;
    }

    let root_discriminant = discriminant.sqrt();
    let root_discriminant = ErrorFloat::new(root_discriminant, root_discriminant * f64::EPSILON * 0.5);
    let q = if b.value < 0. {
      -0.5 * (b - root_discriminant)
    } else {
      -0.5 * (b + root_discriminant)
    };
    let t0 = q / a;
    let t1 = c / q;
    let (t0, t1) = if t0.value <= t1.value { (t0, t1) } else { (t1, t0) };
    Some((t0, t1))
  }
}

impl Into<f64> for ErrorFloat {
  fn into(self) -> f64 {
    self.value
  }
}

impl From<f64> for ErrorFloat {
  fn from(value: f64) -> Self {
    ErrorFloat::new(value, 0.)
  }
}

impl Add for ErrorFloat {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    let value = self.value + other.value;
    let low = (self.low + other.low).next_after(DOWN);
    let high = (self.high + other.high).next_after(UP);
    Self::Output { value, low, high }
  }
}

impl Add<ErrorFloat> for f64 {
  type Output = ErrorFloat;

  fn add(self, other: ErrorFloat) -> Self::Output {
    ErrorFloat::from(self) + other
  }
}

impl Sub for ErrorFloat {
  type Output = Self;
  fn sub(self, other: Self) -> Self::Output {
    let value = self.value - other.value;
    let low = (self.low - other.high).next_after(DOWN);
    let high = (self.high - other.low).next_after(UP);
    Self::Output { value, low, high }
  }
}

impl Sub<ErrorFloat> for f64 {
  type Output = ErrorFloat;
  fn sub(self, other: ErrorFloat) -> Self::Output {
    ErrorFloat::from(self) - other
  }
}

impl Mul for ErrorFloat {
  type Output = Self;

  fn mul(self, other: Self) -> Self::Output {
    let value = self.value * other.value;
    let err_products = [
      self.low * other.low, self.high * other.low,
      self.low * other.high, self.high * other.high,
    ];

    let min = err_products[0].min(err_products[1]).min(err_products[2]).min(err_products[3]);
    let max = err_products[0].max(err_products[1]).max(err_products[2]).max(err_products[3]);
    let low = min.next_after(DOWN);
    let high = max.next_after(UP);
    Self::Output { value, low, high }
  }
}

impl Mul<ErrorFloat> for f64 {
  type Output = ErrorFloat;
  fn mul(self, other: ErrorFloat) -> Self::Output {
    ErrorFloat::from(self) * other
  }
}

impl Div for ErrorFloat {
  type Output = Self;
  fn div(self, other: Self) -> Self::Output {
    let value = self.value / other.value;
    if other.low < 0. && other.high > 0. {
      // The error interval we're dividing by straddles 0,
      // so we'll return a maximally error'd range
      return ErrorFloat { value, low: f64::NEG_INFINITY, high: f64::INFINITY };
    }
    
    let err_quots = [
      self.low / other.low, self.high / other.low,
      self.low / other.high, self.high / other.high,
    ];

    let min = err_quots[0].min(err_quots[1]).min(err_quots[2]).min(err_quots[3]);
    let max = err_quots[0].max(err_quots[1]).max(err_quots[2]).max(err_quots[3]);
    let low = min.next_after(DOWN);
    let high = max.next_after(UP);
    Self::Output { value, low, high }
  }
}

impl Div<ErrorFloat> for f64 {
  type Output = ErrorFloat;
  fn div(self, other: ErrorFloat) -> Self::Output {
    ErrorFloat::from(self) / other
  }
}

impl Neg for ErrorFloat {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self::Output { value: -self.value, low: -self.high, high: -self.low }
  }
}

impl PartialEq for ErrorFloat {
  fn eq(&self, other: &Self) -> bool {
    self.value == other.value
  }
}