use crate::{geometry::{Normal3, Point2, Vector3}, render::{BxDF, BxDFCategory, BxDFSample, Fresnel, Spectrum, shading_coordinates}, scene::TransportMode};


#[derive(Clone)]
pub struct SpecularReflection {
  pub color_scale: Spectrum,
  pub fresnel_properties: Fresnel,
}

impl BxDF for SpecularReflection {
  fn category(&self) -> BxDFCategory {
    BxDFCategory::SPECULAR | BxDFCategory::REFLECTION
  }

  fn evaluate(&self, _outgoing: Vector3<f64>, _incoming: Vector3<f64>) -> Spectrum {
    // For perfect reflection, an arbitrary pair of directions returns no scattering
    // Our sample function will handle picking the perfect incoming angle instead
    Spectrum::default()
  }

  fn sample_function(&self, outgoing: Vector3<f64>, _sample: &Point2<f64>) -> BxDFSample {
    let incoming = Vector3::new(-outgoing.x, -outgoing.y, outgoing.z);
    let probability_distribution = 1.;

    let cos_incident = shading_coordinates::cos_theta(incoming);
    let scale = self.color_scale / cos_incident.abs();
    let value = self.fresnel_properties.evaluate(cos_incident) * scale;

    BxDFSample {
      value,
      incoming,
      probability_distribution,
      category: BxDFCategory::SPECULAR | BxDFCategory::REFLECTION,
    }
  }

  fn probability_distribution(&self, _outgoing: Vector3<f64>, _incoming: Vector3<f64>) -> f64 {
    0.
  }
}


#[derive(Clone)]
pub struct SpecularTransmission {
  pub color: Spectrum,
  pub fresnel_properties: Fresnel,
  pub refraction_indices: (f64, f64),
  pub transport_mode: TransportMode,
}

impl SpecularTransmission {
  pub fn new(color: Spectrum, refraction_indices: (f64, f64), mode: TransportMode) -> Self {
    Self {
      color,
      fresnel_properties: Fresnel::Dialectric {
        eta_parallel: refraction_indices.0,
        eta_perpendicular: refraction_indices.1,
      },
      refraction_indices,
      transport_mode: mode,
    }
  }
}

pub fn refract(outgoing: Vector3, normal: Normal3, refraction: f64) -> Option<Vector3> {
  let normal: Vector3 = normal.into();
  let cos_theta_parallel = normal.dot(outgoing);
  let sin_sq_theta_parallel = (1. - cos_theta_parallel * cos_theta_parallel).max(0.);
  let sin_sq_theta_perpendicular = refraction * refraction * sin_sq_theta_parallel;
  // if the angle is low enough, we experience total internal reflection
  if sin_sq_theta_perpendicular >= 1. { return None; }

  let cos_theta_perpendicular = (1. - sin_sq_theta_perpendicular).sqrt();

  let incoming = -outgoing;
  Some(incoming * refraction + normal * (refraction * cos_theta_parallel - cos_theta_perpendicular))
}

impl BxDF for SpecularTransmission {
  fn category(&self) -> BxDFCategory {
    BxDFCategory::SPECULAR | BxDFCategory::TRANSMISSION
  }

  fn evaluate(&self, _outgoing: Vector3<f64>, _incoming: Vector3<f64>) -> Spectrum {
    // For perfect transmission, an arbitrary pair of directions returns no scattering
    // Our sample function will handle picking the perfect incoming angle instead
    Spectrum::default()
  }

  fn sample_function(&self, outgoing: Vector3<f64>, _sample: &Point2<f64>) -> BxDFSample {
    // Are we entering or leaving this surface, relative to the surface normal?
    let entering = shading_coordinates::cos_theta(outgoing) > 0.;
    let (eta_parallel, eta_perpendicular) = if entering {
      self.refraction_indices
    } else {
      (self.refraction_indices.1, self.refraction_indices.0)
    };

    let normal = Normal3::new(0., 0., 1.).face_with(&outgoing.into());
    let refraction_ratio = eta_parallel / eta_perpendicular;
    let incoming = if let Some(incoming) = refract(outgoing, normal, refraction_ratio) {
      incoming
    } else {
      return BxDFSample {
        value: Spectrum::default(),
        category: BxDFCategory::NONE,
        incoming: Vector3::default(),
        probability_distribution: 1.,
      };
    };
    
    let probability_distribution = 1.;

    let fresnel_value = self.fresnel_properties.evaluate(shading_coordinates::cos_theta(incoming));
    let value = self.color * (Spectrum::white() - fresnel_value);
    let value = if matches!(self.transport_mode, TransportMode::Radiance) {
      value * refraction_ratio * refraction_ratio
    } else {
      value
    };
    
    BxDFSample {
      value,
      incoming,
      probability_distribution,
      category: BxDFCategory::SPECULAR | BxDFCategory::TRANSMISSION,
    }
  }

  fn probability_distribution(&self, _outgoing: Vector3<f64>, _incoming: Vector3<f64>) -> f64 {
    0.
  }
}
