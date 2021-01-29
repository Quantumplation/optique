use std::{path::PathBuf, sync::RwLock};

use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};

use crate::geometry::{Bounds2, Point2};

use super::Spectrum;

pub struct Film {
  pub resolution: Point2<u32>,
  pub pixels: RwLock<Vec<Spectrum>>,
}

impl Film {
  pub fn new(resolution: Point2<u32>) -> Self {
    assert!(resolution.x > 0 && resolution.y > 0, "Must have positive resolution");
    let pixels = RwLock::new(vec![Spectrum::default(); (resolution.x * resolution.y) as usize]);
    Self { resolution, pixels }
  }
  pub fn bounds(&self) -> Bounds2<u32> {
    Bounds2 { min: Default::default(), max: self.resolution }
  }
  pub fn add_sample(&self, pixel: Point2<u32>, value: Spectrum, _weight: f64) {
    let idx = pixel.y * self.resolution.x + pixel.x;
    let mut pixels = self.pixels.write().unwrap();
    pixels[idx as usize] = value;
  }
  pub fn write_to(&self, file: PathBuf) {
    let (width, height) = (self.resolution.x as u32, self.resolution.y as u32);
    let mut img: RgbImage = ImageBuffer::new(width, height);
    let pixels = self.pixels.read().unwrap();
    for y in 0..height {
      for x in 0..width {
        let idx = (y * width + x) as usize;
        let (r,g,b) = (
          (pixels[idx].r * 255.) as u8,
          (pixels[idx].g * 255.) as u8,
          (pixels[idx].b * 255.) as u8,
        );
        img.put_pixel(x, y, Rgb::from([r, g, b]));
      }
    }
    img.save_with_format(file, ImageFormat::Png).unwrap();
  }
}