use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Material {
}

#[enum_dispatch(Material)]
pub enum MaterialInstance {
  NullMaterial,
}

pub struct NullMaterial {
}

impl Material for NullMaterial {
}