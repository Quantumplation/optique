use super::{Point3, Vector3};


pub struct InteractionCommon {
  pub point: Point3<f32>,
  pub reverse_ray: Vector3<f32>,
  pub normal: Vector3<f32>,
}

pub struct SurfaceInteraction {
  pub common: InteractionCommon,
}