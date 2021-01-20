use std::{ops::Mul};

use super::{Matrix4x4, Point3, Ray, Vector3, gamma};

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
  
  pub fn translate(delta: Vector3<f64>) -> Self {
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

  pub fn scale(axis: Vector3<f64>) -> Self {
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
    let field_of_view_radians = field_of_view * (3.141592653 / 180.);
    let half_fov = field_of_view_radians / 2.;
    let inv_tan_angle = 1. / (half_fov).tan();
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

  pub fn look_at(pos: Point3<f64>, look: Point3<f64>, up: Vector3<f64>) -> Self {
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

impl Mul<Vector3<f64>> for Transform {
    type Output = Vector3<f64>;

    fn mul(self, rhs: Vector3<f64>) -> Self::Output {
      let matrix = &self.matrix.m;
      let x  = matrix[0][0] * rhs.x + matrix[0][1] * rhs.y + matrix[0][2] * rhs.z;
      let y  = matrix[1][0] * rhs.x + matrix[1][1] * rhs.y + matrix[1][2] * rhs.z;
      let z  = matrix[2][0] * rhs.x + matrix[2][1] * rhs.y + matrix[2][2] * rhs.z;
      Self::Output { x, y, z }
    }
}

impl Mul<Point3<f64>> for Transform {
  type Output = Point3<f64>;

  fn mul(self, rhs: Point3<f64>) -> Self::Output {
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
  
impl Mul<Ray> for Transform {
  type Output = Ray;
  
  fn mul(self, rhs: Ray) -> Self::Output {
    Ray {
      origin: self * rhs.origin,
      direction: self * rhs.direction,
      time_max: rhs.time_max,
    }
  }
}

pub trait MulWithError<T = Self> {
  type Output;
  type Err;
  fn mul_with_error(&self, other: T) -> (Self::Output, Self::Err);
  fn mul_with_error_in(&self, other: T, err: Self::Err) -> (Self::Output, Self::Err);
}

impl MulWithError<Vector3<f64>> for Transform {
  type Output = Vector3<f64>;
  type Err = Vector3<f64>;
  fn mul_with_error(&self, p: Vector3<f64>) -> (Self::Output, Self::Err) {
    let (x, y, z) = (p.x, p.y, p.z);
    let transformed_point = *self * p;
    let matrix = &self.matrix.m;
    let x_err = (matrix[0][0] * x).abs() + (matrix[0][1] * y).abs() + (matrix[0][2] * z).abs() + (matrix[0][3]).abs();
    let y_err = (matrix[1][0] * x).abs() + (matrix[1][1] * y).abs() + (matrix[1][2] * z).abs() + (matrix[1][3]).abs();
    let z_err = (matrix[2][0] * x).abs() + (matrix[2][1] * y).abs() + (matrix[2][2] * z).abs() + (matrix[2][3]).abs();
    let err = Vector3 { x: x_err, y: y_err, z: z_err } * gamma(3);
    (transformed_point, err)
  }
  fn mul_with_error_in(&self, p: Vector3<f64>, err: Vector3<f64>) -> (Self::Output, Self::Err) {
    let (x, y, z) = (p.x, p.y, p.z);
    let (ex, ey, ez) = (err.x, err.y, err.z);

    let transformed_point = *self * p;

    let matrix = &self.matrix.m;
    let g_0 = gamma(3);
    let g_1 = g_0 + 1.;
    let x_err =
      g_1 *  (matrix[0][0].abs() * ex +  matrix[0][1].abs() * ey +  matrix[0][2].abs() * ez + (matrix[0][3]).abs()) +
      g_0 * ((matrix[0][0] * x).abs() + (matrix[0][1] * y).abs() + (matrix[0][2] * z).abs() + (matrix[0][3].abs()));
    let y_err =
      g_1 *  (matrix[1][0].abs() * ex +  matrix[1][1].abs() * ey +  matrix[1][2].abs() * ez + (matrix[1][3]).abs()) +
      g_0 * ((matrix[1][0] * x).abs() + (matrix[1][1] * y).abs() + (matrix[1][2] * z).abs() + (matrix[1][3].abs()));
    let z_err =
      g_1 *  (matrix[2][0].abs() * ex +  matrix[2][1].abs() * ey +  matrix[2][2].abs() * ez + (matrix[2][3]).abs()) +
      g_0 * ((matrix[2][0] * x).abs() + (matrix[2][1] * y).abs() + (matrix[2][2] * z).abs() + (matrix[2][3].abs()));
    let err = Vector3 { x: x_err, y: y_err, z: z_err };
    (transformed_point, err)
  }
}

impl MulWithError<Point3<f64>> for Transform {
  type Output = Point3<f64>;
  type Err = Point3<f64>;
  fn mul_with_error(&self, other: Point3<f64>) -> (Self::Output, Self::Err) {
    let (v, e) = self.mul_with_error(Vector3::from(other));
    (v.into(), e.into())
  }
  fn mul_with_error_in(&self, other: Point3<f64>, err: Self::Err) -> (Self::Output, Self::Err) {
    let (v, e) = self.mul_with_error_in(Vector3::from(other), Vector3::from(err));
    (v.into(), e.into())
  }
}

impl MulWithError<Ray> for Transform {
  type Output = Ray;
  type Err = Ray;
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
      Ray { origin: origin_err, direction: dir_err, time_max: 0. }
    )
  }

  fn mul_with_error_in(&self, other: Ray, err: Self::Err) -> (Self::Output, Self::Err) {
    let (mut origin, origin_err) = self.mul_with_error_in(other.origin, err.origin);
    let (direction, dir_err) = self.mul_with_error_in(other.direction, err.direction);
    let length_sq = direction.length_squared();
    if length_sq > 0. {
      let offset = direction.abs().dot(Vector3::from(origin_err)) / length_sq;
      origin = origin + direction * offset;
    }
    (
      Ray { origin, direction, time_max: other.time_max },
      Ray { origin: origin_err, direction: dir_err, time_max: 0. }
    )
  }
}