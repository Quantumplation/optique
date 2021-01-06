use crate::geometry::{Bounds3, Ray, SurfaceInteraction};

use super::{Light, LightInstance, NullPrimitive, Primitive, PrimitiveInstance};

#[allow(dead_code)]
pub struct Scene {
  lights: Vec<LightInstance>,
  root: PrimitiveInstance,
  world_bounds: Bounds3<f32>,
}

#[allow(dead_code)]
impl Scene {
  pub fn new(root: PrimitiveInstance, lights: Vec<LightInstance>) -> Scene {
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
      PrimitiveInstance::from(NullPrimitive {}),
      scene.lights.iter().map(LightInstance::from).collect(),
    )
  }

  pub fn intersect(&self, _ray: &Ray) -> Option<SurfaceInteraction> {
    None
  }

  pub fn any_intersect(&self, _ray: &mut Ray) -> bool {
    false
  }
}