
use std::sync::Arc;

use bitflags::bitflags;
use enum_dispatch::enum_dispatch;

use crate::geometry::{INV_PI, Point2, Vector3};
use super::{Spectrum};

bitflags! {
    pub struct BxDFCategory: u8 {
      const NONE         = 0b00000000;
      const REFLECTION   = 0b00000001;
      const TRANSMISSION = 0b00000010;
      const DIFFUSE      = 0b00000100;
      const GLOSSY       = 0b00001000;
      const SPECULAR     = 0b00010000;
      const ALL =
        Self::DIFFUSE.bits | Self::GLOSSY.bits | Self::SPECULAR.bits |
        Self::REFLECTION.bits | Self::TRANSMISSION.bits;
    }
  }
  
  pub struct BxDFSample {
    value: Spectrum,
    incoming: Vector3,
    probability_distribution: f64,
    category: BxDFCategory,
  }
  
  mod shading_coordinates {
    use crate::geometry::Vector3;
  
    pub fn abs_cos_theta(w: Vector3) -> f64 {
      w.z.abs()
    }
  
    pub fn same_hemisphere(a: Vector3, b: Vector3) -> bool {
      a.z * b.z > 0.
    }
  
  }
  
  #[enum_dispatch]
  pub trait BxDF {
    fn category(&self) -> BxDFCategory;
    fn evaluate(&self, outgoing: Vector3, incoming: Vector3) -> Spectrum;
  
    fn sample_function(&self, outgoing: Vector3, sample: &Point2) -> BxDFSample {
      // TODO: hemisphere sampling
      let incoming = Vector3::new(
        sample.x,
        sample.y,
        if outgoing.z < 0. { 1. } else { -1. }
      ).normalized();
  
      let probability_distribution = self.probability_distribution(outgoing, incoming);
      let value = self.evaluate(outgoing, incoming);
  
      BxDFSample {
        value,
        incoming,
        probability_distribution,
        category: BxDFCategory::NONE,
      }
    }
    fn hemispherical_directional_reflectance(&self, outgoing: Vector3, samples: &[Point2]) -> Spectrum {
      let mut result = Spectrum::default();
      for sample in samples {
        let f_sample = self.sample_function(outgoing, sample);
        if f_sample.probability_distribution > 0. {
          result += f_sample.value * shading_coordinates::abs_cos_theta(f_sample.incoming) / f_sample.probability_distribution;
        }
      }
      return result;
    }
    fn hemispherical_hemispherical_reflectance(&self, samples1: &[Point2], samples2: &[Point2]) -> Spectrum {
      let result = Spectrum::default();
      for (_a, _b) in samples1.iter().zip(samples2) {
        // TODO: random sampling
        unimplemented!("Haven't implemented random sampling yet");
      }
      return result;
    }
    fn probability_distribution(&self, outgoing: Vector3, incoming: Vector3) -> f64 {
      if shading_coordinates::same_hemisphere(incoming, outgoing) {
        shading_coordinates::abs_cos_theta(incoming) * INV_PI
      } else {
        0.
      }
    }
  }
  
  #[enum_dispatch(BxDF)]
  pub enum BxDFInstance {
    ScaledBxDF,
  }
  
  pub struct ScaledBxDF {
    pub original: Arc<BxDFInstance>,
    pub scale: Spectrum,
  }
  
  impl BxDF for ScaledBxDF {
    fn category(&self) -> BxDFCategory {
      self.original.category()
    }
    fn evaluate(&self, outgoing: Vector3, incoming: Vector3) -> Spectrum {
      self.scale * self.original.evaluate(outgoing, incoming)
    }
  }