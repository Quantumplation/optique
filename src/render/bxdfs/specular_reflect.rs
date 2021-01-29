use crate::{geometry::{Point2, Vector3}, render::{BxDF, BxDFCategory, BxDFSample, Fresnel, Spectrum, shading_coordinates}};


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