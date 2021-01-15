use std::{ops::Mul};

use super::{Matrix4x4, Point3, Ray, Vector3};

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
  
  pub fn translate(delta: Vector3<f32>) -> Self {
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

  pub fn scale(axis: Vector3<f32>) -> Self {
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
  
  pub fn perspective(field_of_view: f32, near: f32, far: f32) -> Self {
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
}

impl Mul for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
      let m = self.matrix * rhs.matrix;
      let inverse = rhs.inverse * self.inverse;
      Transform::new(m, Some(inverse))
    }
}

impl Mul<Vector3<f32>> for Transform {
    type Output = Vector3<f32>;

    fn mul(self, rhs: Vector3<f32>) -> Self::Output {
      let matrix = &self.matrix.m;
      let x  = matrix[0][0] * rhs.x + matrix[0][1] * rhs.y + matrix[0][2] * rhs.z + matrix[0][3];
      let y  = matrix[1][0] * rhs.x + matrix[1][1] * rhs.y + matrix[1][2] * rhs.z + matrix[1][3];
      let z  = matrix[2][0] * rhs.x + matrix[2][1] * rhs.y + matrix[2][2] * rhs.z + matrix[2][3];
      let wp = matrix[3][0] * rhs.x + matrix[3][1] * rhs.y + matrix[3][2] * rhs.z + matrix[3][3];
      if wp == 1. {
        return Vector3 { x, y, z };
      } else {
        let (x, y, z) = (x / wp, y / wp, z / wp);
        return Vector3 { x, y, z };
      }
    }
}

impl Mul<Point3<f32>> for Transform {
  type Output = Point3<f32>;

  fn mul(self, rhs: Point3<f32>) -> Self::Output {
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
    }
  }
}