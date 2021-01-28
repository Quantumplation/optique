use std::{fs, path::PathBuf};

use crate::{geometry::{Normal3, Point3, Transform, Vector3}, scene::TriangleMesh};

pub fn read_ply(obj_to_world: Transform, file: PathBuf) -> TriangleMesh {
  let file = fs::read_to_string(file).unwrap();
  let v_line = file.lines().nth(3).unwrap();
  let vertex_count: usize = v_line.split(" ").nth(2).unwrap().parse().unwrap();
  let f_line = file.lines().nth(9).unwrap();
  let face_count: usize = f_line.split(" ").nth(2).unwrap().parse().unwrap();

  let mut vertices = vec![];
  let mut normals = vec![];
  let mut tangents = vec![];
  for line in file.lines().skip(12).take(vertex_count) {
    let vertex: Vec<f64> = line.split(" ").map(|p| p.parse::<f64>().unwrap()).take(3).collect();
    let vertex = Point3 { x: vertex[0], y: vertex[1], z: vertex[2] };
    vertices.push(vertex);
    normals.push(Normal3::default());
    tangents.push(Vector3::default());
  }

  let mut indices = vec![];
  for line in file.lines().skip(12 + vertex_count) {
    let parts: Vec<_> = line.split(" ").skip(1).take(3).collect();
    let face: Vec<usize> = parts.into_iter().map(|p| p.parse::<usize>().unwrap()).collect();
    indices.push(face[0]);
    indices.push(face[1]);
    indices.push(face[2]);
  }

  return TriangleMesh::new(
    obj_to_world,
    &indices[..],
    &vertices[..],
    &normals[..],
    &tangents[..],
  );
}