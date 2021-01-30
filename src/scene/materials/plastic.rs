use bumpalo::Bump;

use crate::{geometry::Intersection, render::{BSDF, Fresnel, LambertianReflection, MicrofacetDistributionInstance, MicrofacetReflection, Spectrum, SpecularReflection, TrowbridgeReitz}, scene::{Material, MaterialInstance, TransportMode}};


#[derive(Clone, Copy)]
pub struct Plastic {
  // TODO: textures
  pub diffuse_reflection: Spectrum,
  pub glossy_reflection: Spectrum,
  pub roughness: f64,
  pub remap_roughness: bool,
}

impl From<Plastic> for MaterialInstance {
  fn from(m: Plastic) -> Self {
    MaterialInstance::Plastic(m)
  }
}

impl Material for Plastic {
  fn compute_scattering_functions<'a>(&'a self, intersection: &Intersection, arena: &'a Bump, _mode: TransportMode, _allow_multiple_lobes: bool) -> &'a mut BSDF {
    let bsdf = BSDF::new(arena, intersection, 1.);
    if !self.diffuse_reflection.is_black() {
      let lambert = arena.alloc(LambertianReflection { color: self.diffuse_reflection }.into());
      bsdf.add_component(lambert);
    }

    if self.glossy_reflection.is_black() {
      let roughness = if self.remap_roughness {
        TrowbridgeReitz::roughness_to_azimuth(self.roughness)
      } else {
        self.roughness
      };

      let fresnel = Fresnel::Dialectric { eta_parallel: 1., eta_perpendicular: 1.5, };
      let microfacet_dist = TrowbridgeReitz { azimuthal_x: roughness, azimuthal_y: roughness };
      let microfacet = arena.alloc(MicrofacetReflection {
        color: self.glossy_reflection,
        distribution: microfacet_dist.into(),
        fresnel: fresnel
      }.into());
      bsdf.add_component(microfacet);
    }
    return bsdf;
  }
}