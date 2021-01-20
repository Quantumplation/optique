use crate::{geometry::{Bounds3, Point3, Ray, SurfaceInteraction}, render::Spectrum};

use super::{AreaLight, GeometricPrimitive, Light, LightInstance, Primitive, PrimitiveInstance, ShapeInstance, SphereShape};

#[allow(dead_code)]
pub struct Scene {
  pub lights: Vec<LightInstance>,
  pub root: PrimitiveInstance,
  pub world_bounds: Bounds3<f64>,
}

#[allow(dead_code)]
impl Scene {
  pub fn new(root: PrimitiveInstance, lights: Vec<LightInstance>) -> Scene {
    let world_bounds = root.bounds();
    let mut scene = Scene {
      lights: vec![],
      root,
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
      PrimitiveInstance::from(GeometricPrimitive {
        shape: ShapeInstance::from(SphereShape { point: Point3 { x: 0., y: 0., z: 10. }, radius: 2. }),
        emission: Some(AreaLight { emitted_color: Spectrum { r: 0.3, g: 0., b: 0. } }),
      }),
      scene.lights.iter().map(LightInstance::from).collect(),
    )
  }

  pub fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
    self.root.intersect(&ray)
  }

  pub fn any_intersect(&self, ray: &Ray) -> bool {
    // TODO: optimize
    self.root.intersect(&ray).is_some()
  }
}