use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, Default)]
pub struct Spectrum {
  pub r: f64,
  pub g: f64,
  pub b: f64,
}

pub enum RadianceProblems {
  HasNaNs,
  NegativeLuminance,
  InfiniteLuminance,
}

const LUMINANCE_WEIGHT: [f64; 3] = [0.212_671, 0.715_160, 0.072_169];
impl Spectrum {
  pub fn white() -> Spectrum {
    Spectrum::greyscale(1.)
  }
  pub fn black() -> Spectrum {
    Spectrum::default()
  }
  pub fn greyscale(f: f64) -> Spectrum {
    Spectrum { r: f, g: f, b: f }
  }

  pub fn luminance(&self) -> f64 {
    self.r * LUMINANCE_WEIGHT[0] + self.g * LUMINANCE_WEIGHT[1] + self.b * LUMINANCE_WEIGHT[2]
  }

  pub fn is_valid(&self) -> Option<RadianceProblems> {
    use RadianceProblems::*;
    if self.r.is_nan() || self.g.is_nan() || self.b.is_nan() {
      Some(HasNaNs)
    } else {
      let lum = self.luminance();
      if lum < 0. {
        Some(NegativeLuminance)
      } else if lum.is_infinite() {
        Some(InfiniteLuminance)
      } else {
        None
      }
    }
  }
  
  pub fn is_black(&self) -> bool {
    self.r == 0. && self.g == 0. && self.b == 0.
  }
    
  pub fn sqrt(&self) -> Spectrum {
    Spectrum { r: self.r.sqrt(), g: self.g.sqrt(), b: self.b.sqrt() }
  }
}

impl Add<f64> for Spectrum {
  type Output = Self;
  fn add(self, s: f64) -> Self::Output {
    Self::Output { r: self.r + s, g: self.g + s, b: self.b + s }
  }
}

impl Add for Spectrum {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self::Output { r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b }
  }
}

impl Sub for Spectrum {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    Self::Output { r: self.r - rhs.r, g: self.g - rhs.g, b: self.b - rhs.b }
  }
}

impl AddAssign for Spectrum {
  fn add_assign(&mut self, rhs: Self) {
    self.r += rhs.r;
    self.g += rhs.g;
    self.b += rhs.b;
  }
}

impl Mul<f64> for Spectrum {
  type Output = Self;
  fn mul(self, s: f64) -> Self::Output {
    Self::Output { r: s * self.r, g: s * self.g, b: s * self.b }
  }
}

impl Mul<Spectrum> for f64 {
  type Output = Spectrum;
  fn mul(self, rhs: Spectrum) -> Self::Output {
    rhs * self
  }
}

impl Mul for Spectrum {
  type Output = Self;
  fn mul(self, s: Spectrum) -> Self::Output {
    Self::Output { r: self.r * s.r, g: self.g * s.g, b: self.b * s.b }
  }
}

impl Div<f64> for Spectrum {
  type Output = Self;
  fn div(self, s: f64) -> Self::Output {
    Self::Output { r: self.r / s, g: self.g / s, b: self.b / s }
  }
}

impl Div for Spectrum {
  type Output = Self;
  fn div(self, s: Self) -> Self::Output {
    Self::Output { r: self.r / s.r, g: self.g / s.g, b: self.b / s.b }
  }
}