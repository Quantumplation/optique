use std::sync::Arc;

use crate::{render::Spectrum, scene::{AreaLight, GeometricPrimitive}};

use super::{Point3, Vector3};


pub struct InteractionCommon {
  pub point: Point3<f32>,
  pub distance: f32,
  pub reverse_ray: Vector3<f32>,
  pub normal: Vector3<f32>,
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