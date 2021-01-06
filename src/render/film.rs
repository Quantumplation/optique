use std::path::PathBuf;

use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};

use crate::geometry::{Bounds2, Point2};

pub struct Film {
  pub resolution: Point2<i32>,
  pub pixels: Vec<f32>,
}

impl Film {
  pub fn new(resolution: Point2<i32>) -> Self {
    assert!(resolution.x > 0 && resolution.y > 0, "Must have positive resolution");
    let pixels = vec![0.0; (resolution.x * resolution.y) as usize];
    Self { resolution, pixels }
  }
  pub fn bounds(&self) -> Bounds2<i32> {
    Bounds2 { min: Default::default(), max: self.resolution }
  }
  pub fn add_sample(&mut self, pixel: Point2<i32>, value: f32, weight: f32) {
    let idx = pixel.y * self.resolution.x + pixel.x;
    self.pixels[idx as usize] = value;
  }
  pub fn write_to(&self, file: PathBuf) {
    let (width, height) = (self.resolution.x as u32, self.resolution.y as u32);
    let mut img: RgbImage = ImageBuffer::new(width, height);
    for y in 0..height {
      for x in 0..width {
        let idx = (y * width + x) as usize;
        let pixel = (self.pixels[idx] * 255.) as u8;
        img.put_pixel(x, y, Rgb::from([pixel, pixel, pixel]));
      }
    }
    img.save_with_format(file, ImageFormat::Png).unwrap();
  }
}