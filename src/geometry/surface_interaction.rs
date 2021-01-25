use crate::{render::Spectrum, scene::AreaLight};

use super::{Normal3, Point3, Ray, Vector3};


#[derive(Default, Clone)]
pub struct InteractionCommon {
  pub point: Point3,
  pub point_derivative: (Vector3, Vector3),
  pub reverse_ray: Vector3,
  pub normal: Normal3,
  pub normal_derivative: (Normal3, Normal3),
  pub shading_normal: Normal3,
  pub shading_normal_derivative: (Normal3, Normal3),
  pub intersection_time: f64,
  pub error: Vector3,
}

impl InteractionCommon {
  #[allow(dead_code)]
  pub fn ray_between(&self, other: &InteractionCommon) -> Ray {
    let origin = self.point.offset_for_error(self.error, self.normal, Vector3::from(other.point - self.point));
    let target = other.point.offset_for_error(other.error, other.normal, Vector3::from(origin - other.point));
    let direction = Vector3::from(target - origin).normalized();
    Ray { origin, direction, time_max: f64::INFINITY }
  }
}

pub struct SurfaceInteraction {
  pub common: InteractionCommon,
  pub emissive_properties: Option<AreaLight>,
}

impl SurfaceInteraction {
  pub fn emitted_radiance(&self) -> Spectrum {
    if let Some(emission) = &self.emissive_properties {
      // TOOD(pi): Should this be shading normal?
      emission.emitted_radiance(&self, self.common.normal.into())
    } else {
      Spectrum::default()
    }
  }
}