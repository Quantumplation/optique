use std::cmp::max;

use super::Spectrum;

pub enum Fresnel {
  Conductor { eta_parallel: Spectrum, eta_perpendicular: Spectrum, absorption: Spectrum },
  Dialectric { eta_parallel: f64, eta_perpendicular: f64 },
  NoOp,
}

impl Fresnel {
  pub fn evaluate(&self, cos_incident: f64) -> Spectrum {
    match self {
      &Fresnel::Conductor { eta_parallel, eta_perpendicular, absorption } => fresnel_conductor(cos_incident, eta_parallel, eta_perpendicular, absorption),
      &Fresnel::Dialectric { eta_parallel, eta_perpendicular } => fresnel_dialectric(cos_incident, eta_parallel, eta_perpendicular),
      &Fresnel::NoOp => Spectrum::greyscale(1.),
    }
  }
}

fn fresnel_conductor(
  cos_incident: f64,
  eta_parallel: Spectrum,
  eta_perpendicular: Spectrum,
  absorption: Spectrum
) -> Spectrum {
  let cos_incident = cos_incident.clamp(-1., 1.);
  let eta_ratio: Spectrum = eta_perpendicular / eta_parallel;
  let eta_absorption: Spectrum = absorption / eta_parallel;

  let cos_incident_sq = cos_incident * cos_incident;
  let sin_incident_sq = 1. - cos_incident_sq;
  let eta_ratio_sq: Spectrum = eta_ratio * eta_ratio;
  let eta_absorption_sq: Spectrum = eta_absorption * eta_absorption;
  
  let t_0: Spectrum = eta_ratio_sq - eta_absorption_sq - Spectrum::greyscale(sin_incident_sq);
  let discriminant: Spectrum = (t_0 * t_0 + 4. * eta_ratio_sq * eta_absorption_sq).sqrt();
  let t_1: Spectrum = discriminant + cos_incident_sq;
  let a: Spectrum = (0.5 * (discriminant + t_0)).sqrt();
  let t_2: Spectrum = 2. * cos_incident * a;
  let r_s: Spectrum = (t_1 - t_2) / (t_1 + t_2);

  let t_3: Spectrum = cos_incident_sq * discriminant + sin_incident_sq * sin_incident_sq;
  let t_4: Spectrum = t_2 * sin_incident_sq;
  let r_p: Spectrum = r_s * (t_3 - t_4) / (t_3 + t_4);

  0.5 * (r_p + r_s)
}

fn fresnel_dialectric(
  cos_incident: f64,
  eta_parallel: f64,
  eta_perpendicular: f64,
) -> Spectrum {
  let cos_incident = cos_incident.clamp(-1., 1.);
  let (eta_parallel, eta_perpendicular) = if cos_incident > 0. {
    (eta_perpendicular, eta_parallel)
  } else {
    (eta_parallel, eta_perpendicular)
  };

  let sin_incident_sq = 1. - cos_incident * cos_incident;
  let sin_incident = sin_incident_sq.max(0.).sqrt();
  let sin_perpendicular = eta_parallel / eta_perpendicular * sin_incident;

  if sin_perpendicular >= 1. { return Spectrum::greyscale(1.); }

  let cos_perpendicular_sq = 1. - sin_perpendicular * sin_perpendicular;
  let cos_perpendicular = cos_perpendicular_sq.max(0.).sqrt();

  let r_parallel = 
    ((eta_perpendicular * cos_incident) - (eta_parallel * cos_perpendicular)) /
    ((eta_perpendicular * cos_incident) + (eta_parallel * cos_perpendicular));
  let r_parallel_sq = r_parallel * r_parallel;

  let r_perpendicular =
    ((eta_parallel * cos_incident) - (eta_perpendicular * cos_perpendicular)) /
    ((eta_parallel * cos_incident) + (eta_perpendicular * cos_perpendicular));
  let r_perpendicular_sq = r_perpendicular * r_perpendicular;

  return Spectrum::greyscale((r_parallel_sq + r_perpendicular_sq) / 2.);
}