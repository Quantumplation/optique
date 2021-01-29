use bumpalo::Bump;

use crate::{geometry::{Intersection}, render::{BSDF, BxDFInstance, Fresnel, LambertianReflection, OrenNayar, Spectrum, SpecularTransmission}};

#[derive(Clone)]
pub enum TransportMode {
  Radiance,
  Importance
}

pub trait Material {
    fn compute_scattering_functions<'a>(&'a self, intersection: &Intersection, arena: &'a Bump, transport_mode: TransportMode, allow_multiple_lobes: bool) -> &'a mut BSDF;
}

#[derive(Clone, Copy)]
pub enum MaterialInstance {
  Matte(Matte),
}

impl From<Matte> for MaterialInstance {
  fn from(m: Matte) -> Self {
    MaterialInstance::Matte(m)
  }
}

// Can't use enum_dispatch because of lifetime parameters
impl Material for MaterialInstance {
  fn compute_scattering_functions<'a>(&'a self, intersection: &Intersection, arena: &'a Bump, mode: TransportMode, allow_multiple_lobes: bool) -> &'a mut BSDF {
    match self {
      MaterialInstance::Matte(m) => m.compute_scattering_functions(intersection, arena, mode, allow_multiple_lobes)
    }
  }
}

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
      let lambert = arena.alloc(SpecularTransmission::new(
        self.color,
        (1., 1.5),
        mode
      ).into());
      bsdf.add_component(lambert);
    } else {
      let oren_nayar = arena.alloc(BxDFInstance::from(OrenNayar::new(self.color, self.roughness)));
      bsdf.add_component(oren_nayar);
    }
    return bsdf;
  }
}