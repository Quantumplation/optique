use std::f64::consts::FRAC_1_PI;

use crate::{geometry::{Point2, Vector3}, render::{BxDF, BxDFCategory, Spectrum}};

#[derive(Clone)]
/// Represents light reflection that is perfectly uniformly scattered
pub struct LambertianReflection {
  pub scattered_color: Spectrum,
}

impl BxDF for LambertianReflection {
    fn category(&self) -> BxDFCategory {
        BxDFCategory::REFLECTION | BxDFCategory::DIFFUSE
    }

    fn evaluate(&self, _o: Vector3, _i: Vector3) -> Spectrum {
        self.scattered_color * FRAC_1_PI
    }

    fn hemispherical_directional_reflectance(&self, _o: Vector3, _s: &[Point2]) -> Spectrum {
        self.scattered_color
    }
    fn hemispherical_hemispherical_reflectance(&self, _s1: &[Point2], _s2: &[Point2]) -> Spectrum {
        self.scattered_color
    }
}
