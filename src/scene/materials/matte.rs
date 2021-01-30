use bumpalo::Bump;

use crate::{geometry::Intersection, render::{BSDF, BxDFInstance, LambertianReflection, OrenNayar, Spectrum, SpecularTransmission}, scene::{Material, TransportMode}};


#[derive(Clone, Copy)]
pub struct Matte {
  // TODO: Textures
  pub color: Spectrum,
  pub roughness: f64,
}

impl Material for Matte {
  fn compute_scattering_functions<'a>(&'a self, intersection: &Intersection, arena: &'a Bump, mode: TransportMode, allow_multiple_lobes: bool) -> &'a mut BSDF {
    let bsdf = BSDF::new(arena, intersection, 1.);
    if self.roughness == 0. {
      let lambert = arena.alloc(LambertianReflection { scattered_color: self.color }.into());
      bsdf.add_component(lambert);
    } else {
      let oren_nayar = arena.alloc(BxDFInstance::from(OrenNayar::new(self.color, self.roughness)));
      bsdf.add_component(oren_nayar);
    }
    return bsdf;
  }
}