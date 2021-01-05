use crate::geometry::{Bounds3f, Ray, SurfaceInteraction};

use super::{Light, Primitive};

pub struct Scene {
  lights: Vec<Light>,
  root: Primitive,
  world_bounds: Bounds3f,
}

impl Scene {
  pub fn new(root: Primitive, lights: Vec<Light>) -> Scene {
    let world_bounds = root.bounds();
    let mut scene = Scene {
      lights: vec![],
      root: root.clone(),
      world_bounds,
    };

    for mut light in lights {
      light.preprocess(&scene);
      scene.lights.push(light);
    }

    scene
  }

  pub fn from(scene: &pbrt_rs::Scene) -> Scene {
    Scene::new(
      Primitive::Null,
      scene.lights.iter().map(Light::from).collect(),
    )
  }

  pub fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
    None
  }

  pub fn any_intersect(&self, ray: &mut Ray) -> bool {
    false
  }
}