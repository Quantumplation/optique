use bumpalo::Bump;

use crate::{render::{BSDF, Spectrum}, scene::{AreaLight, Material, MaterialInstance, TransportMode}};

use super::{Normal3, Point3, Ray, Vector3};

#[derive(Clone, Copy, Default)]
pub struct Intersection {
  pub point: Point3,
  pub point_derivative: (Vector3, Vector3),
  pub outgoing: Vector3,
  pub normal: Normal3,
  pub normal_derivative: (Normal3, Normal3),
  pub shading_normal: Normal3,
  pub shading_normal_derivative: (Normal3, Normal3),
  pub distance: f64,
  pub error: Vector3, // TODO: what type of error?
}

pub struct Interaction {
  pub intersection: Intersection,
  pub emission: Option<AreaLight>,
  pub material: Option<MaterialInstance>,
}

impl Intersection {
  pub fn ray_between(&self, other: &Intersection) -> Ray {
    let origin = self.point.offset_for_error(self.error, self.normal, Vector3::from(other.point - self.point));
    let target = other.point.offset_for_error(other.error, other.normal, Vector3::from(origin - other.point));
    let direction = Vector3::from(target - origin).normalized();
    Ray { origin, direction, time_max: f64::INFINITY }
  }
}

impl Interaction {
  pub fn emitted_radiance(&self) -> Spectrum {
    if let Some(emission) = &self.emission {
      // TOOD(pi): Should this be shading normal?
      emission.emitted_radiance(&self.intersection, self.intersection.normal.into())
    } else {
      Spectrum::default()
    }
  }

  pub fn compute_scattering_functions<'a>(&'a self, arena: &'a Bump, mode: TransportMode, multiple_lobes: bool) -> Option<&'a mut BSDF> {
    if let Some(material) = &self.material {
      Some(material.compute_scattering_functions(&self.intersection, arena, mode, multiple_lobes))
    } else {
      None
    }
  }
}