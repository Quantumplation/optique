use super::Scene;

pub enum Light {
  Null,
}

impl Light {
  pub fn from(light: &pbrt_rs::Light) -> Light {
    Light::Null
  }
  
  pub fn preprocess(&mut self, scene: &Scene) {
    
  }
}