use std::{ops::Mul};

use super::{Bounds3, Intersection, Matrix4x4, Normal3, Point3, Ray, TO_RADIANS, Vector3, gamma};

#[derive(Default, Copy, Clone)]
pub struct Transform {
  pub matrix: Matrix4x4,
  pub inverse: Matrix4x4,
}

impl Transform {
  pub fn new(matrix: Matrix4x4, inv: Option<Matrix4x4>) -> Self {
    Transform { inverse: inv.unwrap_or_else(|| matrix.inverse().expect("Non-invertible matrix")), matrix }
  }

  pub fn inverse(&self) -> Self {
    Transform { matrix: self.inverse.clone(), inverse: self.matrix.clone() }
  }
  
  pub fn translate(delta: Vector3) -> Self {
    let m = 
      [[1., 0., 0., delta.x],
       [0., 1., 0., delta.y],
       [0., 0., 1., delta.z],
       [0., 0., 0., 1.]
      ];
    let inv =
      [[1., 0., 0., -delta.x],
       [0., 1., 0., -delta.y],
       [0., 0., 1., -delta.z],
       [0., 0., 0., 1.]
      ];
    Transform::new(Matrix4x4::new(m), Some(Matrix4x4::new(inv)))
  }
  
  pub fn rotate(angle_degrees: f64, axis: Vector3) -> Transform {
    let axis = axis.normalized();
    let sin_theta = (angle_degrees * TO_RADIANS).sin();
    let cos_theta = (angle_degrees * TO_RADIANS).cos();
    let rotated_x = [
      axis.x * axis.x + (1. - axis.x * axis.x) * cos_theta,
      axis.x * axis.y * (1. - cos_theta) - axis.z * sin_theta,
      axis.x * axis.z * (1. - cos_theta) + axis.y * sin_theta,
      0.
    ];
    let rotated_y = [
      axis.x * axis.y * (1. - cos_theta) + axis.z * sin_theta,
      axis.y * axis.y + (1. - axis.y * axis.y) * cos_theta,
      axis.y * axis.z * (1. - cos_theta) - axis.x * sin_theta,
      0.,
    ];
    let rotated_z = [
      axis.x * axis.z * (1. - cos_theta) - axis.y * sin_theta,
      axis.y * axis.z * (1. - cos_theta) + axis.x * sin_theta,
      axis.z * axis.z + (1. - axis.z * axis.z) * cos_theta,
      0.,
    ];
    let m = [
      rotated_x,
      rotated_y,
      rotated_z,
      [0., 0., 0., 1.],
    ];
    Transform::new(Matrix4x4::new(m), Some(Matrix4x4::transpose(m)))
  }

  pub fn scale(axis: Vector3) -> Self {
    let m = 
      [[axis.x,     0.,     0.,      0.],
       [    0., axis.y,     0.,      0.],
       [    0.,     0., axis.z,      0.],
       [    0.,     0.,     0.,      1.]
      ];
    let inv = 
      [[1. / axis.x,          0.,          0.,           0.],
       [         0., 1. / axis.y,          0.,           0.],
       [         0.,          0., 1. / axis.z,           0.],
       [         0.,          0.,          0.,           1.]
      ];
    Transform::new(Matrix4x4::new(m), Some(Matrix4x4::new(inv)))
  }
  
  pub fn perspective(field_of_view: f64, near: f64, far: f64) -> Self {
    let field_of_view_radians = field_of_view * TO_RADIANS;
    let half_fov = field_of_view_radians / 2.;
    let inv_tan_angle = 1. / half_fov.tan();
    let f1 = far / (far - near);
    let f2 = -near * f1;
    let projection = Matrix4x4::from_parts(
      1., 0., 0., 0.,
      0., 1., 0., 0.,
      0., 0., f1, f2,
      0., 0., 1., 0.,
    );

    return Transform::scale(Vector3::new(inv_tan_angle, inv_tan_angle, 1.)) * Transform::new(projection, None);
  }

  pub fn look_at(pos: Point3, look: Point3, up: Vector3) -> Self {
    let mut camera_to_world = Matrix4x4::default();
    let m = &mut camera_to_world.m;
    m[0][3] = pos.x;
    m[1][3] = pos.y;
    m[2][3] = pos.z;
    m[3][3] = 1.;

    let dir = Vector3::from(look - pos).normalized();

    let right = up.normalized().cross(dir).normalized();
    let new_up = dir.cross(right);

    m[0][0] = right.x;
    m[1][0] = right.y;
    m[2][0] = right.z;
    m[3][0] = 0.;

    m[0][1] = new_up.x;
    m[1][1] = new_up.y;
    m[2][1] = new_up.z;
    m[3][1] = 0.;

    m[0][2] = dir.x;
    m[1][2] = dir.y;
    m[2][2] = dir.z;
    m[3][2] = 0.;

    let world_to_camera = camera_to_world.inverse().unwrap();

    Transform::new(world_to_camera, Some(camera_to_world))
  }
}

impl Mul for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
      let m = self.matrix * rhs.matrix;
      let inverse = rhs.inverse * self.inverse;
      Transform::new(m, Some(inverse))
    }
}

impl Mul<Vector3> for Transform {
  type Output = Vector3;

  fn mul(self, rhs: Vector3) -> Self::Output {
    let matrix = &self.matrix.m;
    let x  = matrix[0][0] * rhs.x + matrix[0][1] * rhs.y + matrix[0][2] * rhs.z;
    let y  = matrix[1][0] * rhs.x + matrix[1][1] * rhs.y + matrix[1][2] * rhs.z;
    let z  = matrix[2][0] * rhs.x + matrix[2][1] * rhs.y + matrix[2][2] * rhs.z;
    Self::Output { x, y, z }
  }
}

impl Mul<Point3> for Transform {
  type Output = Point3;

  fn mul(self, rhs: Point3) -> Self::Output {
    let matrix = &self.matrix.m;
    let x  = matrix[0][0] * rhs.x + matrix[0][1] * rhs.y + matrix[0][2] * rhs.z + matrix[0][3];
    let y  = matrix[1][0] * rhs.x + matrix[1][1] * rhs.y + matrix[1][2] * rhs.z + matrix[1][3];
    let z  = matrix[2][0] * rhs.x + matrix[2][1] * rhs.y + matrix[2][2] * rhs.z + matrix[2][3];
    let wp = matrix[3][0] * rhs.x + matrix[3][1] * rhs.y + matrix[3][2] * rhs.z + matrix[3][3];
    if wp == 1. {
      return Point3 { x, y, z };
    } else {
      let (x, y, z) = (x / wp, y / wp, z / wp);
      return Point3 { x, y, z };
    }
  }
}

impl Mul<Normal3> for Transform {
  type Output = Normal3;

  fn mul(self, rhs: Normal3) -> Self::Output {
    // Normals transform contravariantly, i.e. with respect to the inverse transpose
    // in order to stay perpendicular to the transformed surface
    let inv = &self.inverse.m;
    let x = inv[0][0] * rhs.x + inv[1][0] * rhs.y + inv[2][0] * rhs.z;
    let y = inv[0][1] * rhs.x + inv[1][1] * rhs.y + inv[2][1] * rhs.z;
    let z = inv[0][2] * rhs.x + inv[1][2] * rhs.y + inv[2][2] * rhs.z;

    Self::Output { x, y, z }
  }
}
  
impl Mul<Ray> for Transform {
  type Output = Ray;
  
  fn mul(self, rhs: Ray) -> Self::Output {
    let (origin, o_error) = self.mul_with_error(rhs.origin);
    let direction = self * rhs.direction;

    let length_squared = direction.length_squared();
    let origin = if length_squared <= 0. {
      origin
    } else {
      let distance = direction.abs().dot(o_error) / length_squared;
      origin + direction * distance
    };

    Ray {
      origin,
      direction,
      time_max: rhs.time_max,
    }
  }
}

impl Mul<Intersection> for Transform {
  type Output = Intersection;

  fn mul(self, i: Intersection) -> Self::Output {
    let (point, error) = self.mul_with_error_in(i.point, i.error);
    let normal = (self * i.normal).normalized();
    Self::Output {
      point, error,
      point_derivative: (self * i.point_derivative.0, self * i.point_derivative.1),
      outgoing: (self * i.outgoing).normalized(),
      normal,
      normal_derivative: (self * i.normal_derivative.0, self * i.normal_derivative.1),
      shading_normal: (self * i.shading_normal).face_with(&normal),
      shading_normal_derivative: (self * i.shading_normal_derivative.0, self * i.shading_normal_derivative.1),
      distance: i.distance, // TODO: this isn't technically correct
    }
  }
}

impl Mul<Bounds3> for Transform {
  type Output = Bounds3;
  fn mul(self, rhs: Bounds3) -> Self::Output {
    // Transform each corner of the bounding box, and enclose all of them
    let min = self * rhs.min;
    Bounds3::new(min, min)
      .encompass(self * Point3::new(rhs.max.x, rhs.min.y, rhs.min.z))
      .encompass(self * Point3::new(rhs.min.x, rhs.max.y, rhs.min.z))
      .encompass(self * Point3::new(rhs.min.x, rhs.min.y, rhs.max.z))
      .encompass(self * Point3::new(rhs.min.x, rhs.max.y, rhs.max.z))
      .encompass(self * Point3::new(rhs.max.x, rhs.max.y, rhs.min.z))
      .encompass(self * Point3::new(rhs.max.x, rhs.min.y, rhs.max.z))
      .encompass(self * rhs.max)
  }
}

pub trait MulWithError<T = Self> {
  type Output;
  type Err;
  fn mul_with_error(&self, other: T) -> (Self::Output, Self::Err);
  fn mul_with_error_in(&self, other: T, err: Self::Err) -> (Self::Output, Self::Err);
}

impl MulWithError<Point3> for Transform {
  type Output = Point3;
  type Err = Vector3;
  fn mul_with_error(&self, p: Point3) -> (Self::Output, Self::Err) {
    let (x, y, z) = (p.x, p.y, p.z);
    let transformed_point = *self * p;
    let matrix = &self.matrix.m;
    let x_err = (matrix[0][0] * x).abs() + (matrix[0][1] * y).abs() + (matrix[0][2] * z).abs() + (matrix[0][3]).abs();
    let y_err = (matrix[1][0] * x).abs() + (matrix[1][1] * y).abs() + (matrix[1][2] * z).abs() + (matrix[1][3]).abs();
    let z_err = (matrix[2][0] * x).abs() + (matrix[2][1] * y).abs() + (matrix[2][2] * z).abs() + (matrix[2][3]).abs();
    let err = Vector3 { x: x_err, y: y_err, z: z_err } * gamma(3);
    (transformed_point, err)
  }
  fn mul_with_error_in(&self, p: Point3, err: Vector3) -> (Self::Output, Self::Err) {
    let (x, y, z) = (p.x, p.y, p.z);
    let (ex, ey, ez) = (err.x, err.y, err.z);

    let transformed_point = *self * p;

    let matrix = &self.matrix.m;
    let g_0 = gamma(3);
    let g_1 = g_0 + 1.;
    let x_err =
      g_1 *  (matrix[0][0].abs() * ex +  matrix[0][1].abs() * ey +  matrix[0][2].abs() * ez) +
      g_0 * ((matrix[0][0] * x).abs() + (matrix[0][1] * y).abs() + (matrix[0][2] * z).abs() + matrix[0][3].abs());
    let y_err =
      g_1 *  (matrix[1][0].abs() * ex +  matrix[1][1].abs() * ey +  matrix[1][2].abs() * ez) +
      g_0 * ((matrix[1][0] * x).abs() + (matrix[1][1] * y).abs() + (matrix[1][2] * z).abs() + matrix[1][3].abs());
    let z_err =
      g_1 *  (matrix[2][0].abs() * ex +  matrix[2][1].abs() * ey +  matrix[2][2].abs() * ez) +
      g_0 * ((matrix[2][0] * x).abs() + (matrix[2][1] * y).abs() + (matrix[2][2] * z).abs() + matrix[2][3].abs());
    let err = Vector3 { x: x_err, y: y_err, z: z_err };
    (transformed_point, err)
  }
}

impl MulWithError<Vector3> for Transform {
  type Output = Vector3;
  type Err = Vector3;
  fn mul_with_error(&self, p: Vector3) -> (Self::Output, Self::Err) {
    let (x, y, z) = (p.x, p.y, p.z);
    let transformed_point = *self * p;
    let matrix = &self.matrix.m;
    let x_err = (matrix[0][0] * x).abs() + (matrix[0][1] * y).abs() + (matrix[0][2] * z).abs();
    let y_err = (matrix[1][0] * x).abs() + (matrix[1][1] * y).abs() + (matrix[1][2] * z).abs();
    let z_err = (matrix[2][0] * x).abs() + (matrix[2][1] * y).abs() + (matrix[2][2] * z).abs();
    let err = Vector3 { x: x_err, y: y_err, z: z_err } * gamma(3);
    (transformed_point, err)
  }
  fn mul_with_error_in(&self, p: Vector3, err: Self::Err) -> (Self::Output, Self::Err) {
    let (x, y, z) = (p.x, p.y, p.z);
    let (ex, ey, ez) = (err.x, err.y, err.z);

    let transformed_point = *self * p;

    let matrix = &self.matrix.m;
    let g_0 = gamma(3);
    let g_1 = g_0 + 1.;
    let x_err =
      g_1 *  (matrix[0][0].abs() * ex +  matrix[0][1].abs() * ey +  matrix[0][2].abs() * ez) +
      g_0 * ((matrix[0][0] * x).abs() + (matrix[0][1] * y).abs() + (matrix[0][2] * z).abs());
    let y_err =
      g_1 *  (matrix[1][0].abs() * ex +  matrix[1][1].abs() * ey +  matrix[1][2].abs() * ez) +
      g_0 * ((matrix[1][0] * x).abs() + (matrix[1][1] * y).abs() + (matrix[1][2] * z).abs());
    let z_err =
      g_1 *  (matrix[2][0].abs() * ex +  matrix[2][1].abs() * ey +  matrix[2][2].abs() * ez) +
      g_0 * ((matrix[2][0] * x).abs() + (matrix[2][1] * y).abs() + (matrix[2][2] * z).abs());
    let err = Vector3 { x: x_err, y: y_err, z: z_err };
    (transformed_point, err)
  }
}

impl MulWithError<Ray> for Transform {
  type Output = Ray;
  type Err = (Vector3, Vector3);
  fn mul_with_error(&self, other: Ray) -> (Self::Output, Self::Err) {
    let (mut origin, origin_err) = self.mul_with_error(other.origin);
    let (direction, dir_err) = self.mul_with_error(other.direction);
    let length_sq = direction.length_squared();
    if length_sq > 0. {
      let offset = direction.abs().dot(Vector3::from(origin_err)) / length_sq;
      origin = origin + direction * offset;
    }
    (
      Ray { origin, direction, time_max: other.time_max },
      (origin_err, dir_err)
    )
  }

  fn mul_with_error_in(&self, other: Ray, err: Self::Err) -> (Self::Output, Self::Err) {
    let (mut origin, origin_err) = self.mul_with_error_in(other.origin, err.0);
    let (direction, dir_err) = self.mul_with_error_in(other.direction, err.1);
    let length_sq = direction.length_squared();
    if length_sq > 0. {
      let offset = direction.abs().dot(Vector3::from(origin_err)) / length_sq;
      origin = origin + direction * offset;
    }
    (
      Ray { origin, direction, time_max: other.time_max },
      (origin_err, dir_err)
    )
  }
}