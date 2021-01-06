use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Camera {}

#[enum_dispatch(Camera)]
pub enum CameraInstance {
  NullCamera,
}

pub struct NullCamera {}

impl Camera for NullCamera {}