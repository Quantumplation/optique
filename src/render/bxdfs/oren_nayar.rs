use std::f64::consts::FRAC_1_PI;

use crate::{geometry::{TO_RADIANS, Vector3}, render::{BxDF, BxDFCategory, Spectrum}};


#[derive(Clone)]
/// Oren-Nayar Microfacet simulation, based on a uniform distribution of v-shaped microfacets
pub struct OrenNayar {
  pub color: Spectrum,
  /// From the Oren-Nayar microfacet equations
  a: f64,
  /// From the Oren-Nayar microfacet equations
  b: f64,
}

impl OrenNayar {
  pub fn new(color: Spectrum, angle_distribution: f64) -> Self {
    // Precompute a and b, so we don't have to compute them every evaluation
    let sigma = angle_distribution * TO_RADIANS;
    let sigma_sq = sigma * sigma;
    let a = 1. - (sigma_sq / (2. * (sigma + 0.33)));
    let b = 0.45 * sigma_sq / (sigma_sq + 0.09);

    OrenNayar { color, a, b }
  }
}

impl BxDF for OrenNayar {
  fn category(&self) -> BxDFCategory {
    BxDFCategory::REFLECTION | BxDFCategory::DIFFUSE
  }

  fn evaluate(&self, outgoing: Vector3, incoming: Vector3) -> Spectrum {
    use crate::render::shading_coordinates::*;
    let sin_theta_incoming = sin_theta(incoming);
    let sin_theta_outgoing = sin_theta(outgoing);

    let max_cos = if sin_theta_incoming > 1e-4 && sin_theta_outgoing > 1e-4 {
      let sin_phi_incoming = sin_phi(incoming);
      let cos_phi_incoming = cos_phi(incoming);
      let sin_phi_outgoing = sin_phi(outgoing);
      let cos_phi_outgoing = cos_phi(outgoing);

      let d_cos = cos_phi_incoming * cos_phi_outgoing + sin_phi_incoming * sin_phi_outgoing;
      d_cos.max(0.)
    } else {
      0.
    };

    let deviation_incoming = abs_cos_theta(incoming);
    let deviation_outgoing = abs_cos_theta(outgoing);
    let (sin_alpha, tan_beta) = if deviation_incoming > deviation_outgoing {
      (sin_theta_outgoing, sin_theta_incoming / deviation_incoming)
    } else {
      (sin_theta_incoming, sin_theta_outgoing / deviation_outgoing)
    };

    let scatter_amount: f64 = FRAC_1_PI * (self.a + self.b * max_cos * sin_alpha * tan_beta);
    return scatter_amount * self.color;
  }
}