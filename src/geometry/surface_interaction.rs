use crate::{render::Spectrum, scene::AreaLight};

use super::{Point3, Ray, Vector3};


#[derive(Default, Clone)]
pub struct InteractionCommon {
  pub point: Point3<f64>,
  pub distance: f64,
  pub reverse_ray: Vector3<f64>,
  pub normal: Vector3<f64>,
}

impl InteractionCommon {
  #[allow(dead_code)]
  pub fn ray_between(&self, other: &InteractionCommon) -> Ray {
    let origin = self.point;
    let direction = Vector3::from(other.point - self.point);
    Ray { origin, direction }
  }
}

pub struct SurfaceInteraction {
  pub common: InteractionCommon,
  pub emissive_properties: Option<AreaLight>,
}

impl SurfaceInteraction {
  pub fn emitted_radiance(&self) -> Spectrum {
    if let Some(emission) = &self.emissive_properties {
      emission.emitted_radiance(&self, self.common.normal)
    } else {
      Spectrum::default()
    }
  }
}