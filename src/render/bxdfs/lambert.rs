use std::f64::consts::FRAC_1_PI;

use crate::{geometry::{Point2, Vector3}, render::{BxDF, BxDFCategory, BxDFSample, Spectrum, shading_coordinates}};

#[derive(Clone)]
/// Represents light reflection that is perfectly uniformly scattered
pub struct LambertianReflection {
  pub color: Spectrum,
}

impl BxDF for LambertianReflection {
    fn category(&self) -> BxDFCategory {
        BxDFCategory::REFLECTION | BxDFCategory::DIFFUSE
    }

    fn evaluate(&self, _o: Vector3, _i: Vector3) -> Spectrum {
        self.color * FRAC_1_PI
    }

    fn hemispherical_directional_reflectance(&self, _o: Vector3, _s: &[Point2]) -> Spectrum {
        self.color
    }
    fn hemispherical_hemispherical_reflectance(&self, _s1: &[Point2], _s2: &[Point2]) -> Spectrum {
        self.color
    }
}

#[derive(Clone)]
pub struct LambertianTransmission {
    pub color: Spectrum,
}

impl BxDF for LambertianTransmission {
    fn category(&self) -> BxDFCategory {
        BxDFCategory::TRANSMISSION | BxDFCategory::DIFFUSE
    }

    fn evaluate(&self, _o: Vector3, _i: Vector3) -> Spectrum {
        self.color * FRAC_1_PI
    }

    fn sample_function(&self, outgoing: Vector3, sample: &Point2) -> BxDFSample {
        // TODO: proper sampling
        let z = if outgoing.z > 0. { -1. } else { 1. };
        let incoming = Vector3::new(sample.x, sample.y, z).normalized();
        let pdf = self.probability_distribution(outgoing, incoming);
        return BxDFSample {
            value: self.evaluate(outgoing, incoming),
            probability_distribution: pdf,
            incoming,
            category: self.category(),
        };
    }

    fn probability_distribution(&self, outgoing: Vector3, incoming: Vector3) -> f64 {
        if !shading_coordinates::same_hemisphere(outgoing, incoming) {
            shading_coordinates::abs_cos_theta(incoming) * FRAC_1_PI
        } else {
            0.
        }
    }
}