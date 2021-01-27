use crate::{geometry::{Intersection, Point2, Point3, Ray, Vector3}, render::Spectrum};

use super::Scene;
use enum_dispatch::enum_dispatch;

#[derive(Default)]
pub struct RadianceSample {
  pub color: Spectrum,
  pub incident_direction: Vector3,
  pub probability_distribution: f64,
  pub intersections: (Intersection, Intersection)
}

#[enum_dispatch]
pub trait Light {
  fn preprocess(&mut self, scene: &Scene);
  fn power(&self) -> Spectrum;
  fn background_radiance(&self, ray: &Ray) -> Spectrum; // pbrt: Le()
  fn sample_radiance(&self, interaction: &Intersection, point: Point2) -> RadianceSample; // pbrt: Sample_Li()
}

#[enum_dispatch(Light)]
pub enum LightInstance {
  NullLight,
  PointLight,
  AreaLight,
}

impl From<&pbrt_rs::Light> for LightInstance {
    fn from(_: &pbrt_rs::Light) -> Self {
      LightInstance::from(NullLight {})
    }
}

pub struct NullLight {}
impl Light for NullLight {
  fn preprocess(&mut self, _scene: &Scene) {}
  fn power(&self) -> Spectrum { Spectrum::default() }
  fn background_radiance(&self, _: &Ray) -> Spectrum { Spectrum::default() }
  fn sample_radiance(&self, _: &Intersection, _: Point2) -> RadianceSample {
    RadianceSample::default()
  }
}

pub struct PointLight {
  pub position: Point3,
  pub color: Spectrum,
}

impl Light for PointLight {
  fn preprocess(&mut self, _: &Scene) {}
  fn power(&self) -> Spectrum { self.color * 4. * 3.141592 }
  fn background_radiance(&self, _: &Ray) -> Spectrum { Spectrum { r: 0.1, g: 0.1, b: 0.1 } }
  fn sample_radiance(&self, intersection: &Intersection, _: Point2) -> RadianceSample {
    let offset = Vector3::from(self.position - intersection.point);
    let incident_direction = offset.normalized();
    let color = self.color / offset.length_squared();
    let light_interaction = Intersection {
      point: self.position,
      distance: offset.length(),
      ..Default::default()
    };
    return RadianceSample {
      color,
      incident_direction,
      probability_distribution: 1.,
      intersections: (intersection.clone(), light_interaction),
    }
  }
}

#[derive(Clone)]
pub struct AreaLight {
  pub emitted_color: Spectrum,
}

impl Light for AreaLight {
    fn preprocess(&mut self, _scene: &Scene) {}

    fn power(&self) -> Spectrum { Spectrum::default() }
    fn background_radiance(&self, _ray: &Ray) -> Spectrum { Spectrum { r: 1., g: 1., b: 1. } }
    fn sample_radiance(&self, _: &Intersection, _point: Point2) -> RadianceSample { RadianceSample::default() }
}

impl AreaLight {
  // pbrt: L()
  pub fn emitted_radiance(&self, intersection: &Intersection, direction: Vector3) -> Spectrum {
    // TODO: This is actually DiffuseAreaLight
    // TODO: should this be shading normal?
    if direction.dot(intersection.normal.into()) > 0. {
      self.emitted_color.clone()
    } else {
      Spectrum::default()
    }
  }
}