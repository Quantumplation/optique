#[derive(Debug, Copy, Clone, Default)]
pub struct Spectrum {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

pub enum RadianceProblems {
  HasNaNs,
  NegativeLuminance,
  InfiniteLuminance,
}

const LUMINANCE_WEIGHT: [f32; 3] = [0.212_671, 0.715_160, 0.072_169];
impl Spectrum {
  pub fn luminance(&self) -> f32 {
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
}