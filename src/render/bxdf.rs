use std::{sync::Arc};

use bitflags::bitflags;
use enum_dispatch::enum_dispatch;

use super::{Fresnel, Spectrum};
use crate::geometry::{INV_PI, Normal3, Point2, SurfaceInteraction, Vector3};

mod shading_coordinates {
  use crate::geometry::Vector3;

  pub fn cos_theta(w: Vector3) -> f64 {
    w.z
  }

  pub fn abs_cos_theta(w: Vector3) -> f64 {
    w.z.abs()
  }

  pub fn same_hemisphere(a: Vector3, b: Vector3) -> bool {
    a.z * b.z > 0.
  }
}

pub const MAX_BXDF: usize = 8;
/// Bidirectional Scattering Distribution Function
/// Represents the data needed to compute how light scatters on a surface
pub struct BSDF {
  /// A relative index describing how much light bends at the boundary
  /// Should be 1 for opaque objects
  index_of_refraction: f64,
  /// The normal according to the geometry
  geometric_normal: Normal3,
  /// The normal for purposes of shading, such as via a bump-map
  shading_normal: Normal3,
  /// A tangent vector `s`, one component of an orthonormal basis
  tangent_s: Vector3,
  /// A tangent vector, `t`, one component of an orthonormal basis
  tangent_t: Vector3,
  /// The number of surface properties that have been added
  num_components: usize,
  /// The components of the scattering function
  components: [Option<BxDFInstance>; MAX_BXDF],
}

impl BSDF {
  pub fn new(interaction: &SurfaceInteraction, index_of_refraction: f64) -> Self {
    let shading_normal = interaction.common.shading_normal;
    let tangent_s = interaction.common.shading_normal_derivative.0.normalized();
    let tangent_t = shading_normal.cross(tangent_s);
    Self {
      index_of_refraction,
      geometric_normal: interaction.common.normal,
      shading_normal,
      tangent_s: tangent_s.into(),
      tangent_t: tangent_t.into(), 
      num_components: 0,
      components: Default::default(),
    }
  }
}

struct BxDFIterator<'a> {
  components: &'a [Option<BxDFInstance>],
  curr: usize,
  category: BxDFCategory
}

impl<'a> Iterator for BxDFIterator<'a> {
  type Item = &'a BxDFInstance;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      if self.curr >= self.components.len() {
        return None;
      } else if let Some(component) = &self.components[self.curr] {
        // Increment for the next time through before returning
        self.curr += 1;
        if self.category.contains(component.category()) {
          return Some(&component);
        } else {
          continue;
        }
      } else {
        return None;
      }
    }
  }
}

impl BSDF {
  pub fn evaluate(&self, outgoing_world: Vector3, incoming_world: Vector3, category: BxDFCategory) -> Spectrum {
    let incoming = self.transform_world_to_local(incoming_world);
    let outgoing = self.transform_world_to_local(outgoing_world);

    // Use the geometric normal to tell if we're doing reflection or transmission
    // (i.e. if things are in the same hemisphere) to prevent light leak
    // but use the shading normal for actual sampling
    let is_reflection =
      incoming_world.dot(self.geometric_normal.into()) *
      outgoing_world.dot(self.geometric_normal.into()) > 0.;

    self.evaluate_local(outgoing, incoming, category, is_reflection)
  }

  pub fn sample_function(&self, outgoing_world: Vector3, sample: &Point2, category: BxDFCategory) -> BxDFSample {
    let empty_sample = BxDFSample {
      value: Spectrum::default(),
      incoming: Vector3::default(),
      category: BxDFCategory::NONE,
      probability_distribution: 0.,
    };

    let matching_components = self.matching_components(category).count();
    if matching_components == 0 {
      return empty_sample;
    }

    // Uniformly choose a component for this sample
    let chosen_idx = ((sample.x * matching_components as f64).floor() as usize).min(self.num_components);
    
    let component = self.matching_components(category).nth(chosen_idx);
    if component.is_none() {
      return empty_sample;
    }
    let component = component.unwrap();

    let sample_remapped = Point2::new(
      (sample.x * matching_components as f64 - chosen_idx as f64).min(1. - f64::EPSILON),
      sample.y,
    );

    let outgoing = self.transform_world_to_local(outgoing_world);
    if outgoing.z == 0. {
      return empty_sample;
    }

    let sample: BxDFSample = component.sample_function(outgoing, &sample_remapped);

    if sample.probability_distribution == 0. {
      return empty_sample;
    }

    let incoming_world = self.transform_local_to_world(sample.incoming);

    let mut pdf = sample.probability_distribution;
    // If we chose something other than specular,
    // we may need to weight the sample appropriately among other components
    if !component.category().contains(BxDFCategory::SPECULAR) && matching_components > 1 {
      pdf = self.probability_distribution_local(outgoing, sample.incoming, category, (chosen_idx, pdf));
    }
    
    let mut value = sample.value;
    // If we're non-specular, sum up the contributions from other components
    if !component.category().contains(BxDFCategory::SPECULAR) {
      let is_reflection = 
        incoming_world.dot(self.geometric_normal.into()) *
        outgoing_world.dot(self.geometric_normal.into()) > 0.;
      value = self.evaluate_local(outgoing, sample.incoming, category, is_reflection);
    }

    return BxDFSample {
      value,
      incoming: incoming_world,
      category: sample.category,
      probability_distribution: pdf,
    };
  }

  pub fn hemispherical_directional_reflectance(&self, outgoing_world: Vector3, samples: &[Point2], category: BxDFCategory) -> Spectrum {
    let outgoing = self.transform_world_to_local(outgoing_world);
    let mut result = Spectrum::default();
    for component in self.matching_components(category) {
      result += component.hemispherical_directional_reflectance(outgoing, samples);
    }
    return result;
  }

  pub fn hemispherical_hemispherical_reflectance(&self, samples1: &[Point2], samples2: &[Point2], category: BxDFCategory) -> Spectrum {
    let mut result = Spectrum::default();
    for component in self.matching_components(category) {
      result += component.hemispherical_hemispherical_reflectance(samples1, samples2);
    }
    return result;
  }

  pub fn probability_distribution(&self, outgoing_world: Vector3, incoming_world: Vector3, category: BxDFCategory) -> f64 {
    let outgoing = self.transform_world_to_local(outgoing_world);
    let incoming = self.transform_world_to_local(incoming_world);
    self.probability_distribution_local(outgoing, incoming, category, (self.components.len(), 0.))
  }

  fn evaluate_local(&self, outgoing: Vector3, incoming: Vector3, category: BxDFCategory, is_reflection: bool) -> Spectrum {
    if outgoing.z == 0. {
      return Spectrum::default();
    }

    let mut result = Spectrum::default();

    for component in self.matching_components(category) {
      let component_category = component.category();
      let is_reflective = component_category.contains(BxDFCategory::REFLECTION);
      let is_transmissive = component_category.contains(BxDFCategory::TRANSMISSION);

      if (is_reflection && is_reflective) || (!is_reflective && is_transmissive) {
        result += component.evaluate(outgoing, incoming);
      }
    }

    return result;
  }

  fn probability_distribution_local(&self, outgoing: Vector3, incoming: Vector3, category: BxDFCategory, precomputed: (usize, f64)) -> f64 {
    let mut pdf = 0.;
    let mut count = 0;
    for (idx, other) in self.matching_components(category).enumerate() {
      count += 1;
      if idx == precomputed.0 {
        pdf += precomputed.1;
        continue;
      }
      pdf += other.probability_distribution(outgoing, incoming);
    }
  
    if count > 1 {
      pdf /= count as f64;
    }
    return pdf;
  }

  pub fn add_component(&mut self, bxdf: BxDFInstance) {
    self.components[self.num_components] = Some(bxdf);
    self.num_components += 1;
  }

  fn matching_components(&self, category: BxDFCategory) -> BxDFIterator {
    BxDFIterator {
      curr: 0,
      components: &self.components[..],
      category
    }
  }

  fn transform_world_to_local(&self, v: Vector3) -> Vector3 {
    let x = v.dot(self.tangent_s);
    let y = v.dot(self.tangent_t);
    let z = v.dot(self.shading_normal.into());

    Vector3 { x, y, z }
  }

  fn transform_local_to_world(&self, v: Vector3) -> Vector3 {
    let (s, t, n) = (self.tangent_s, self.tangent_t, self.shading_normal);
    let x = s.x * v.x + t.x * v.y + n.x * v.z;
    let y = s.y * v.x + t.y * v.y + n.y * v.z;
    let z = s.z * v.x + t.z * v.y + n.z * v.z;

    Vector3 { x, y, z }
  }
}

bitflags! {
  pub struct BxDFCategory: u8 {
    const NONE         = 0b00000000;
    const REFLECTION   = 0b00000001;
    const TRANSMISSION = 0b00000010;
    const DIFFUSE      = 0b00000100;
    const GLOSSY       = 0b00001000;
    const SPECULAR     = 0b00010000;
    const ALL =
    Self::DIFFUSE.bits | Self::GLOSSY.bits | Self::SPECULAR.bits |
    Self::REFLECTION.bits | Self::TRANSMISSION.bits;
  }
}
pub struct BxDFSample {
  value: Spectrum,
  incoming: Vector3,
  probability_distribution: f64,
  category: BxDFCategory,
}

#[enum_dispatch]
pub trait BxDF {
  fn category(&self) -> BxDFCategory;
  fn evaluate(&self, outgoing: Vector3, incoming: Vector3) -> Spectrum;

  fn sample_function(&self, outgoing: Vector3, sample: &Point2) -> BxDFSample {
    // TODO: hemisphere sampling
    let incoming =
        Vector3::new(sample.x, sample.y, if outgoing.z < 0. { 1. } else { -1. }).normalized();

    let probability_distribution = self.probability_distribution(outgoing, incoming);
    let value = self.evaluate(outgoing, incoming);

    BxDFSample {
        value,
        incoming,
        probability_distribution,
        category: BxDFCategory::NONE,
    }
  }
  fn hemispherical_directional_reflectance(
    &self,
    outgoing: Vector3,
    samples: &[Point2],
  ) -> Spectrum {
    let mut result = Spectrum::default();
    for sample in samples {
        let f_sample = self.sample_function(outgoing, sample);
        if f_sample.probability_distribution > 0. {
            result += f_sample.value * shading_coordinates::abs_cos_theta(f_sample.incoming)
                / f_sample.probability_distribution;
        }
    }
    return result;
  }
  fn hemispherical_hemispherical_reflectance(
    &self,
    samples1: &[Point2],
    samples2: &[Point2],
  ) -> Spectrum {
    let result = Spectrum::default();
    for (_a, _b) in samples1.iter().zip(samples2) {
        // TODO: random sampling
        unimplemented!("Haven't implemented random sampling yet");
    }
    return result;
  }
  fn probability_distribution(&self, outgoing: Vector3, incoming: Vector3) -> f64 {
    if shading_coordinates::same_hemisphere(incoming, outgoing) {
      shading_coordinates::abs_cos_theta(incoming) * INV_PI
    } else {
      0.
    }
  }
}

#[enum_dispatch(BxDF)]
#[derive(Clone)]
pub enum BxDFInstance {
  ScaledBxDF,
  SpecularReflection,
}

#[derive(Clone)]
pub struct ScaledBxDF {
  pub original: Arc<BxDFInstance>,
  pub scale: Spectrum,
}

impl BxDF for ScaledBxDF {
  fn category(&self) -> BxDFCategory {
    self.original.category()
  }
  fn evaluate(&self, outgoing: Vector3, incoming: Vector3) -> Spectrum {
    self.scale * self.original.evaluate(outgoing, incoming)
  }
}

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
