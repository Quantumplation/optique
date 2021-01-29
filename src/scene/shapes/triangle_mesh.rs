use std::sync::Arc;

use crate::{geometry::{Bounds3, Intersection, Normal3, Point3, Ray, Transform, Vector3, gamma}, scene::{Shape, ShapeInstance}};

pub struct TriangleMesh {
  pub indices: Vec<usize>,
  pub vertices: Vec<Point3>,
  pub normals: Vec<Normal3>,
  pub tangents: Vec<Vector3>,
}

impl TriangleMesh {
  pub fn new(
    object_to_world: Transform,
    idx: &[usize],
    vs: &[Point3],
    ns: &[Normal3],
    ts: &[Vector3],
  ) -> Self {

    let indices = idx.to_vec();
    let mut vertices = Vec::with_capacity(vs.len());
    let mut normals = Vec::with_capacity(ns.len());
    let mut tangents = Vec::with_capacity(ts.len());

    // Transform the data to world coordinates, to save transforming many many rays
    for i in 0..vs.len() {
      vertices.push(object_to_world * vs[i]);
      normals.push(object_to_world * ns[i]);
      tangents.push(object_to_world * ts[i]);
    }

    Self { indices, vertices, normals, tangents }
  }

  pub fn to_triangles(self: Arc<Self>) -> Vec<ShapeInstance> {
    let mut result: Vec<ShapeInstance> = vec![];
    for i in (0..self.indices.len()).step_by(3) {
      result.push(TriangleShape {
        mesh: self.clone(),
        index: i
      }.into());
    }
    return result;
  }
}

pub struct TriangleShape {
  mesh: Arc<TriangleMesh>,
  index: usize,
}

impl TriangleShape {
  fn vertices(&self) -> (Point3, Point3, Point3) {
    (
      self.mesh.vertices[self.mesh.indices[self.index    ]],
      self.mesh.vertices[self.mesh.indices[self.index + 1]],
      self.mesh.vertices[self.mesh.indices[self.index + 2]]
    )
  }
}
                      
impl Shape for TriangleShape {
  fn object_to_world(&self) -> Transform {
      Transform::default() // We're already in world coordiantes
  }
  fn bounds(&self) -> Bounds3 {
    todo!();
  }
  fn world_bounds(&self) -> Bounds3 {
    let (p0, p1, p2) = self.vertices();
    Bounds3::new(p0, p0)
      .encompass(p1)
      .encompass(p2)
  }

  fn intersect(&self, ray: &crate::geometry::Ray) -> Option<crate::geometry::Intersection> {
    let (p0, p1, p2) = self.vertices();

    // The intersection test below is built off first translating each vertex point + the ray
    // so that the ray starts at (0,0,0), and points down the z axis
    // This allows us to just test whether (0,0) is inside the projected triangle
    // and also ensures that our raytracing will be water-tight when butted against other triangles
    let ray_origin = Vector3::from(ray.origin);
    // First, translate each point so the ray origin is at (0,0,0)
    let (p0t, p1t, p2t) = (p0 - ray_origin, p1 - ray_origin, p2 - ray_origin);
    // Permute the axis so that "z" has the largest value on the ray
    let (r, mut p0t, mut p1t, mut p2t) = {
      let rd = ray.direction;
      let time_max = ray.time_max;
      let origin = Point3::default();
      if ray.direction.x.abs() > ray.direction.y.abs() {
        if ray.direction.x.abs() > ray.direction.z.abs() {
          // X is the longest dimension, so permute x to z
          (
            Ray { direction: Vector3::new(rd.y, rd.z, rd.x), origin, time_max },
            Point3::new(p0t.y, p0t.z, p0t.x),
            Point3::new(p1t.y, p1t.z, p1t.x),
            Point3::new(p2t.y, p2t.z, p2t.x),
          )
        } else {
          // z is already the longest dimension, so do nothing
          (Ray { direction: ray.direction, origin, time_max }, p0t, p1t, p2t)
        }
      } else if ray.direction.y.abs() > ray.direction.z.abs() {
        // y is the longest dimension
        (
          Ray { direction: Vector3::new(rd.z, rd.x, rd.y), origin, time_max },
          Point3::new(p0t.z, p0t.x, p0t.y),
          Point3::new(p1t.z, p1t.x, p1t.y),
          Point3::new(p2t.z, p2t.x, p2t.y),
        )
      } else {
        // z is already the longest dimension, so do nothing
        (Ray { direction: ray.direction, origin, time_max }, p0t, p1t, p2t)
      }
    };
    // Now apply a shear transform to align the ray with the z axis
    let shear_x = -r.direction.x / r.direction.z;
    let shear_y = -r.direction.y / r.direction.z;
    let shear_z = 1. / r.direction.z;

    p0t.x += shear_x * p0t.z;
    p0t.y += shear_y * p0t.z;
    p1t.x += shear_x * p1t.z;
    p1t.y += shear_y * p1t.z;
    p2t.x += shear_x * p2t.z;
    p2t.y += shear_y * p2t.z;

    // We can now use the cross product to derive a "which side of this edge is 0,0 on" coefficient for each edge
    let e0 = p1t.x * p2t.y - p1t.y * p2t.x;
    let e1 = p2t.x * p0t.y - p2t.y * p0t.x;
    let e2 = p0t.x * p1t.y - p0t.y * p1t.x;

    // Now, either all three of these need to be positive, or all three negative, in order to intersect
    if (e0 < 0. || e1 < 0. || e2 < 0.) && (e0 > 0. || e1 > 0. || e2 > 0.) {
      // The point must lie outside the triangle
      return None;
    }
    // Additionally, if the ray is approaching edge-on, then the sum of these will be 0:
    let determinant = e0 + e1 + e2;
    if determinant == 0. {
      return None;
    }
    
    // Compute the hit distance
    p0t.z *= shear_z;
    p1t.z *= shear_z;
    p2t.z *= shear_z;
    let scaled_t = e0 * p0t.z + e1 * p1t.z + e2 * p2t.z;
    // NOTE: save the cost of a division by comparing the signs, first
    if determinant < 0. && (scaled_t >= 0. || scaled_t < r.time_max * determinant) {
      return None;
    } else if determinant > 0. && (scaled_t <= 0. || scaled_t > r.time_max * determinant) {
      return None;
    }

    // We definitely have an intersection, so now we need to find the intersection values
    let inv_determinant = 1. / determinant;
    let b0 = e0 * inv_determinant;
    let b1 = e1 * inv_determinant;
    let b2 = e2 * inv_determinant;
    let time_hit = scaled_t * inv_determinant;
    
    // Compute the error bounds on the intersection point
    let x_err = (b0 * p0.x).abs() + (b1 * p1.x).abs() + (b2 * p2.x).abs();
    let y_err = (b0 * p0.y).abs() + (b1 * p1.y).abs() + (b2 * p2.y).abs();
    let z_err = (b0 * p0.z).abs() + (b1 * p1.z).abs() + (b2 * p2.z).abs();
    let error: Vector3 = Vector3::new(x_err, y_err, z_err) * gamma(7);
    
    
    let point_hit: Point3 = p0 * b0 + p1 * b1 + p2 * b2;

    let dp02 = Vector3::from(p0 - p2);
    let dp12 = Vector3::from(p1 - p2);

    let normal = Normal3::from(dp02.cross(dp12).normalized());

    return Some(Intersection {
      point: point_hit,
      error,
      distance: time_hit,
      normal: normal,
      normal_derivative: (Normal3::default(), Normal3::default()),
      outgoing: -ray.direction,
      shading_normal: normal,
      shading_normal_derivative: (Normal3::default(), Normal3::default()),
      ..Default::default()
    });
  }
}