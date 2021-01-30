use bumpalo::Bump;

use crate::{geometry::Intersection, render::{BSDF, Fresnel, FresnelSpecular, MicrofacetReflection, MicrofacetTransmission, Spectrum, SpecularReflection, SpecularTransmission, TrowbridgeReitz}, scene::{Material, MaterialInstance, TransportMode}};


#[derive(Clone, Copy)]
pub struct Glass {
  // TODO: textures
  pub color_reflected: Spectrum,
  pub color_transmitted: Spectrum,
  pub roughness: (f64, f64),
  pub refraction: f64,
  pub remap_roughness: bool,
}

impl From<Glass> for MaterialInstance {
  fn from(g: Glass) -> Self {
    MaterialInstance::Glass(g)
  }
}

impl Material for Glass {
  fn compute_scattering_functions<'a>(&'a self, intersection: &Intersection, arena: &'a Bump, mode: TransportMode, allow_multiple_lobes: bool) -> &'a mut BSDF {
    let bsdf = BSDF::new(arena, intersection, self.refraction);

    if self.color_reflected.is_black() && self.color_transmitted.is_black() {
      return bsdf;
    }

    let is_specular = self.roughness.0 == 0. && self.roughness.1 == 0.;
    if is_specular && allow_multiple_lobes {
      let fresnel = arena.alloc(FresnelSpecular {
        color_reflected: self.color_reflected,
        color_transmitted: self.color_transmitted,
        refraction: (1., self.refraction),
        mode,
      }.into());
      bsdf.add_component(fresnel);
    } else {
      let roughness = if self.remap_roughness {
        (
          TrowbridgeReitz::roughness_to_azimuth(self.roughness.0),
          TrowbridgeReitz::roughness_to_azimuth(self.roughness.1),
        )
      } else {
        self.roughness
      };

      let distrib = if is_specular {
        None
      } else {
        Some(TrowbridgeReitz { azimuthal_y: roughness.0, azimuthal_x: roughness.1 })
      };

      if !self.color_reflected.is_black() {
        let fresnel = Fresnel::Dialectric { eta_parallel: 1., eta_perpendicular: self.refraction };
        if is_specular {
          let reflection = arena.alloc(SpecularReflection {
            color_scale: self.color_reflected,
            fresnel_properties: fresnel,
          }.into());
          bsdf.add_component(reflection);
        } else {
          let microfacet = arena.alloc(MicrofacetReflection {
            color: self.color_reflected,
            fresnel,
            distribution: distrib.clone().unwrap().into(),
          }.into());
          bsdf.add_component(microfacet);
        }
      }

      if !self.color_transmitted.is_black() {
        if is_specular {
          let transmission = arena.alloc(SpecularTransmission::new(
            self.color_transmitted,
            (1., self.refraction),
            mode
          ).into());
          bsdf.add_component(transmission);
        } else {
          let microfacet = arena.alloc(MicrofacetTransmission {
            color: self.color_transmitted,
            distribution: distrib.unwrap().into(),
            fresnel: Fresnel::Dialectric { eta_parallel: 1., eta_perpendicular: self.refraction },
            refraction: (1., self.refraction),
            mode,
          }.into());
          bsdf.add_component(microfacet);
        }
      }
    }

    return bsdf;
  }
}