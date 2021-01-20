use std::sync::Arc;

use enum_dispatch::enum_dispatch;

use crate::geometry::{Bounds2, Point3, Ray, RayDifferential, Transform, Vector3};

use super::{CameraSample, Film};

#[enum_dispatch]
pub trait Camera {
    fn bounds(&self) -> Bounds2<i32>;
    fn film(&self) -> Arc<Film>;
    fn generate_ray(&self, sample: &CameraSample) -> (f64, Ray);

    fn generate_ray_differential(&self, sample: &CameraSample) -> (f64, RayDifferential) {
      // Generate 3 rays:
      // - the one we'd normally generate
      let (wt, ray) = self.generate_ray(sample);
      
      // - shifted by one pixel in the x direction
      let (wtx, ray_x) = {
        let mut sample = sample.clone();
        sample.film_point.x += 1.;
        self.generate_ray(&sample)
      };

      // - shifted by one pixel in the y direction
      let (wty, ray_y) = {
        let mut sample = sample.clone();
        sample.film_point.y += 1.;
        self.generate_ray(&sample)
      };
      // This is used for anti-aliasing, for example

      // Make sure to use the weight from the first ray, but if either of our differentials were 0, use that
      // TODO: Not sure why the book does this
      let wt = if wtx == 0. || wty == 0. { 0. } else { wt };
      (wt, RayDifferential { ray, ray_x, ray_y })
    }
}

#[enum_dispatch(Camera)]
pub enum CameraInstance {
  PerspectiveCamera,
}

pub struct PerspectiveCamera {
  pub film: Arc<Film>,
  pub camera_to_world: Transform,
  pub camera_to_screen: Transform,
  pub raster_to_camera: Transform,
  pub screen_to_raster: Transform,
  pub raster_to_screen: Transform,
  pub shutter_open: f64,
  pub shutter_close: f64,
  pub lens_radius: f64,
  pub focal_distance: f64,
  pub pixel_ray_dx: Vector3<f64>,
  pub pixel_ray_dy: Vector3<f64>,
  pub view_area: f64,
}

impl PerspectiveCamera {
  pub fn new(
    camera_to_world: Transform, bounds: Bounds2<f64>,
    shutter_open: f64, shutter_close: f64, lens_radius: f64, focal_distance: f64,
    field_of_view: f64,
    film: Arc<Film>
  ) -> Self {
    let camera_to_screen = Transform::perspective(field_of_view, 0.01, 1000.);

    let resolution = film.bounds().max;
    let resolution_scale = Transform::scale(Vector3::new(resolution.x as f64, resolution.y as f64, 1.));
    let screen_scale = Transform::scale(Vector3::new(
      1. / (bounds.max.x - bounds.min.x),
      1. / (bounds.max.y - bounds.min.y),
      1.
    ));
    let translate = Transform::translate(Vector3::new(-bounds.min.x, -bounds.min.y, 0.));
    let screen_to_raster = resolution_scale * screen_scale * translate;
    let raster_to_screen = screen_to_raster.inverse();
    let raster_to_camera = camera_to_screen.inverse() * raster_to_screen;

    let zero = Vector3::default();
    let dx = Vector3::new(1., 0., 0.);
    let dy = Vector3::new(0., 1., 0.);
    let pixel_ray_dx: Vector3<_> = (raster_to_camera * dx) - (raster_to_camera * zero);
    let pixel_ray_dy: Vector3<_> = (raster_to_camera * dy) - (raster_to_camera * zero);
    
    let camera_min: Vector3<_> = raster_to_camera * zero;
    let camera_max: Vector3<_> = raster_to_camera * Vector3::new(resolution.x as f64, resolution.y as f64, 0.);
    let near_min: Vector3<_> = camera_min / camera_min.z;
    let near_max: Vector3<_> = camera_max / camera_max.z;
    let view_area = ((near_max.x - near_min.x) * (near_max.y - near_min.y)).abs();

    PerspectiveCamera {
      film,
      shutter_open, shutter_close,
      lens_radius, focal_distance,
      camera_to_world,
      camera_to_screen,
      screen_to_raster,
      raster_to_screen,
      raster_to_camera,
      pixel_ray_dx,
      pixel_ray_dy,
      view_area,
    }
  }
}

impl Camera for PerspectiveCamera {
  fn bounds(&self) -> Bounds2<i32> {
    self.film.bounds()
  }
  fn film(&self) -> Arc<Film> {
    self.film.clone()
  }
  fn generate_ray(&self, sample: &CameraSample) -> (f64, Ray) {
    let point_raster = Point3::new(sample.film_point.x, sample.film_point.y, 0.);
    let point_camera = self.raster_to_camera * point_raster;
    let direction = Vector3::from(point_camera).normalized();
    let ray = Ray { origin: Point3::default(), direction, time_max: f64::INFINITY };
    if self.lens_radius > 0. {
      unimplemented!("Depth of field is not implemented yet");
    }
    let ray = self.camera_to_world * ray;
    (1., ray)
  }
  fn generate_ray_differential(&self, sample: &CameraSample) -> (f64, RayDifferential) {
      if self.lens_radius > 0. {
        unimplemented!("Depth of field is not implemented yet");
      } else {
        // NOTE: reimplements generate_ray above, because we need to reuse point_camera
        let point_raster = Point3::new(sample.film_point.x, sample.film_point.y, 0.);
        let point_camera = self.raster_to_camera * point_raster;
        let direction = Vector3::from(point_camera).normalized();
        let ray = self.camera_to_world * Ray { origin: Point3::default(), direction, time_max: f64::INFINITY, };
        let ray_x = self.camera_to_world * Ray { origin: ray.origin, direction: Vector3::from(point_camera + self.pixel_ray_dx).normalized(), time_max: f64::INFINITY };
        let ray_y = self.camera_to_world * Ray { origin: ray.origin, direction: Vector3::from(point_camera + self.pixel_ray_dy).normalized(), time_max: f64::INFINITY };

        return (1., RayDifferential {
          ray,
          ray_x,
          ray_y,
        });
      }
  }
}