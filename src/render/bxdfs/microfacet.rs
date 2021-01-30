use std::{f64::consts::PI};

use enum_dispatch::enum_dispatch;
use shading_coordinates::{abs_cos_theta, cos_sq_theta, cos_theta, same_hemisphere, sin_sq_phi, tan_sq_theta, tan_theta};

use crate::{geometry::{Vector3}, render::{BxDF, BxDFCategory, Fresnel, Spectrum, shading_coordinates::{self, cos_sq_phi}}, scene::TransportMode};

#[enum_dispatch]
pub trait MicrofacetDistribution {
  fn area_distribution(&self, normal: Vector3) -> f64;
  fn masked_facet_ratio(&self, outgoing: Vector3) -> f64;

  fn mask_shadow_factor(&self, outgoing: Vector3) -> f64 {
    1. / (1. + self.masked_facet_ratio(outgoing))
  }
  fn dual_visible_factor(&self, outgoing: Vector3, incoming: Vector3) -> f64 {
    1. / (1. + self.mask_shadow_factor(outgoing) + self.mask_shadow_factor(incoming))
  }
}
#[enum_dispatch(MicrofacetDistribution)]
#[derive(Clone)]
pub enum MicrofacetDistributionInstance {
  Beckmann,
  TrowbridgeReitz,
}

#[derive(Clone)]
pub struct Beckmann {
  azimuthal_x: f64,
  azimuthal_y: f64,
}

impl MicrofacetDistribution for Beckmann {
  fn area_distribution(&self, normal: Vector3) -> f64 {
    let tan_sq_theta = tan_sq_theta(normal);
    if tan_sq_theta.is_infinite() {
      return 0.;
    }

    let cos_fourth_theta = cos_sq_theta(normal) * cos_sq_theta(normal);
    let ax_sq = self.azimuthal_x * self.azimuthal_x;
    let ay_sq = self.azimuthal_y * self.azimuthal_y;
    let exponent = -tan_sq_theta * (cos_sq_phi(normal) / ax_sq + sin_sq_phi(normal) / ay_sq);
    let denom = PI * self.azimuthal_x * self.azimuthal_y * cos_fourth_theta;

    return exponent.exp() / denom;
  }

  fn masked_facet_ratio(&self, outgoing: Vector3) -> f64 {
    let abs_tan_theta = tan_theta(outgoing).abs();
    if abs_tan_theta.is_infinite() { return 0. }

    let ax_sq = self.azimuthal_x * self.azimuthal_x;
    let ay_sq = self.azimuthal_y * self.azimuthal_y;

    let interpolated_azimuth = (cos_sq_phi(outgoing) * ax_sq + sin_sq_phi(outgoing) * ay_sq).sqrt();

    let a = 1. / (interpolated_azimuth * abs_tan_theta);
    if a >= 1.6 { return 0. }

    // Approximation of general function, which uses the expensive exp and erf functions
    return (1. - 1.259 * a + 0.396 * a * a)
         / (     3.535 * a + 2.181 * a * a);
  }
}

#[derive(Clone)]
pub struct TrowbridgeReitz {
  pub azimuthal_x: f64,
  pub azimuthal_y: f64,
}

impl MicrofacetDistribution for TrowbridgeReitz {
  fn area_distribution(&self, normal: Vector3) -> f64 {
    let abs_tan_theta = tan_sq_theta(normal);
    if abs_tan_theta.is_infinite() { return 0.; }

    let cos_fourth_theta = cos_sq_theta(normal) * cos_sq_theta(normal);
    let ax_sq = self.azimuthal_x * self.azimuthal_x;
    let ay_sq = self.azimuthal_y * self.azimuthal_y;
    let exp = (cos_sq_phi(normal) / ax_sq + sin_sq_phi(normal) / ay_sq) / abs_tan_theta;
    return 1. / (PI * self.azimuthal_x * self.azimuthal_y * cos_fourth_theta * (1. + exp) * (1. + exp));
  }

  fn masked_facet_ratio(&self, outgoing: Vector3) -> f64 {
    let abs_tan_theta = tan_sq_theta(outgoing);
    if abs_tan_theta.is_infinite() { return 0.; }

    let ax_sq = self.azimuthal_x * self.azimuthal_x;
    let ay_sq = self.azimuthal_y * self.azimuthal_y;

    let interpolated_azimuth = (cos_sq_phi(outgoing) * ax_sq + sin_sq_phi(outgoing) * ay_sq).sqrt();

    let alpha_sq_tan_sq_theta = (interpolated_azimuth * abs_tan_theta)  * (interpolated_azimuth * abs_tan_theta);
    return (-1. + (1. + alpha_sq_tan_sq_theta).sqrt()) / 2.;
  }
}

impl TrowbridgeReitz {
  pub fn roughness_to_azimuth(roughness: f64) -> f64 {
    let roughness = roughness.max(1e-3);
    let log_rough = roughness.ln();
    let log_rough_sq = log_rough * log_rough;
    let log_rough_cub = log_rough_sq * log_rough;
    let log_rough_fourth = log_rough_sq * log_rough_sq;
    // quartic Approximation polynomial
    return 1.62142
         + 0.819955 * log_rough
         + 0.1734 * log_rough_sq
         + 0.0171201 * log_rough_cub
         + 0.000640711 * log_rough_fourth;
  }
}

#[derive(Clone)]
pub struct MicrofacetReflection {
  pub color: Spectrum,
  pub distribution: MicrofacetDistributionInstance,
  pub fresnel: Fresnel,
}

impl BxDF for MicrofacetReflection {
  fn category(&self) -> BxDFCategory {
    BxDFCategory::REFLECTION | BxDFCategory::GLOSSY
  }

  fn evaluate(&self, outgoing: Vector3, incoming: Vector3) -> Spectrum {
    let cos_theta_outgoing = abs_cos_theta(outgoing);
    let cos_theta_incoming = abs_cos_theta(incoming);

    let half_angle_vec: Vector3 = outgoing + incoming;

    // In certain degenerate cases, we shouldn't reflect any light
    if cos_theta_incoming == 0. || cos_theta_outgoing == 0. { return Spectrum::black(); }
    if half_angle_vec.x == 0. && half_angle_vec.y == 0. && half_angle_vec.z == 0. { return Spectrum::black(); }

    let half_angle_vec = half_angle_vec.normalized();

    let fresnel_scale = self.fresnel.evaluate(incoming.dot(outgoing));
    let area_distribution = self.distribution.area_distribution(half_angle_vec);
    let visible_factor = self.distribution.dual_visible_factor(outgoing, incoming);
    let energy_conservation = 4. * cos_theta_incoming * cos_theta_outgoing;

    return self.color * fresnel_scale * area_distribution * visible_factor / energy_conservation;
  }
}

#[derive(Clone)]
pub struct MicrofacetTransmission {
  pub color: Spectrum,
  pub distribution: MicrofacetDistributionInstance,
  pub refraction: (f64, f64),
  pub fresnel: Fresnel,
  pub mode: TransportMode,
}

impl BxDF for MicrofacetTransmission {
  fn category(&self) -> BxDFCategory {
    BxDFCategory::TRANSMISSION | BxDFCategory::GLOSSY
  }

  fn evaluate(&self, outgoing: Vector3, incoming: Vector3) -> Spectrum {
    if same_hemisphere(outgoing, incoming) { return Spectrum::black(); }

    let cos_theta_outgoing = cos_theta(outgoing);
    let cos_theta_incoming = cos_theta(incoming);

    if cos_theta_incoming == 0. || cos_theta_outgoing == 0. { return Spectrum::black(); }

    let refraction = if cos_theta_outgoing > 0. {
      self.refraction.1 / self.refraction.0
    } else {
      self.refraction.0 / self.refraction.1
    };

    let split_vec = (outgoing + incoming * refraction).normalized();

    let out_dot_split = outgoing.dot(split_vec);
    let in_dot_split = incoming.dot(split_vec);
    if out_dot_split * in_dot_split > 0. { return Spectrum::black(); }

    let refraction_sq = refraction * refraction;

    let color_scale = self.fresnel.evaluate(out_dot_split);

    let sqrt_denom = out_dot_split + in_dot_split * refraction;
    let factor = if matches!(self.mode, TransportMode::Radiance) { 1. / refraction } else { 1. };
    let factor_sq = factor * factor;
    let area_distribution = self.distribution.area_distribution(split_vec);
    let visible_factor = self.distribution.dual_visible_factor(outgoing, incoming);
    let energy_conservation = cos_theta_incoming * cos_theta_incoming * sqrt_denom * sqrt_denom;
    
    let transmitted_light =
      area_distribution
       * visible_factor
       * refraction_sq
       * in_dot_split * out_dot_split
       * factor_sq
       / energy_conservation;
    return (Spectrum::white() - color_scale) * self.color * transmitted_light;
  }
}