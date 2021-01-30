use bumpalo::Bump;

use crate::{geometry::Intersection, render::{BSDF, Fresnel, Spectrum, SpecularReflection}, scene::{Material, MaterialInstance, TransportMode}};


#[derive(Clone, Copy)]
pub struct Mirror {
  // TODO: textures
  pub color: Spectrum,
}

impl From<Mirror> for MaterialInstance {
  fn from(m: Mirror) -> Self {
    MaterialInstance::Mirror(m)
  }
}

impl Material for Mirror {
  fn compute_scattering_functions<'a>(&'a self, intersection: &Intersection, arena: &'a Bump, _mode: TransportMode, _allow_multiple_lobes: bool) -> &'a mut BSDF {
    let bsdf = BSDF::new(arena, intersection, 1.);
    if !self.color.is_black() {
      let fresnel_properties = Fresnel::NoOp;
      let reflection = arena.alloc(SpecularReflection { color_scale: self.color, fresnel_properties }.into());
      bsdf.add_component(reflection);
    }
    return bsdf;
  }
}