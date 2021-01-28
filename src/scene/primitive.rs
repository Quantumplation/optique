
use std::{borrow::Borrow, ops::Range, sync::Arc};

use bumpalo::Bump;
use enum_dispatch::enum_dispatch;
use crate::{geometry::{Bounds3, Interaction, Point3, Ray, Vector3}};

use super::{AreaLight, MaterialInstance, Shape, ShapeInstance};
#[enum_dispatch]
pub trait Primitive {
  fn world_bounds(&self) -> Bounds3<f64>;
  fn intersect(&self, ray: &Ray) -> Option<Interaction>;
}

#[enum_dispatch(Primitive)]
pub enum PrimitiveInstance {
  NullPrimitive,
  GeometricPrimitive,
  PrimitiveList,
  BVHAggregate
}

pub struct NullPrimitive {}

impl Primitive for NullPrimitive {
  fn world_bounds(&self) -> Bounds3<f64> {
    Bounds3::default()
  }
  fn intersect(&self, _ray: &Ray) -> Option<Interaction> {
    None
  }
}

pub struct GeometricPrimitive {
  pub shape: ShapeInstance,
  pub material: Option<MaterialInstance>,
  pub emission: Option<AreaLight>,
}

impl Primitive for GeometricPrimitive {
  fn world_bounds(&self) -> Bounds3<f64> {
    self.shape.world_bounds()
  }

  fn intersect(&self, ray: &Ray) -> Option<Interaction> {
    self.shape.intersect(ray).map(|intersection| {
      Interaction {
        intersection,
        emission: self.emission.clone(),
        material: self.material.clone(),
      }
    }) 
  }
}

pub struct PrimitiveList {
  pub primitives: Vec<PrimitiveInstance>,
}

impl Primitive for PrimitiveList {
    fn world_bounds(&self) -> Bounds3<f64> {
      Bounds3::default() // TODO
    }

    fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        let mut min_dist = -1.;
        let mut min_interaction = None;
        for p in &self.primitives {
          if let Some(i) = p.intersect(ray) {
            if min_dist < 0. || i.intersection.distance < min_dist {
              min_dist = i.intersection.distance;
              min_interaction = Some(i);
            }
          }
        }
        min_interaction
    }
}

pub enum SplitMethod { SurfaceArea, Linear, Middle, EqualCounts }
pub struct BVHAggregate {
  max_node_size: usize,
  split_method: SplitMethod,
  primitives: Vec<PrimitiveInstance>,
  nodes: Vec<BVHNode>,
}

/// NOTE: this enum should be exactly 64 bytes to align with cache lines
pub enum BVHNode {
  Interior { bounds: Bounds3, second_child: u32, axis: u8 },
  Leaf { bounds: Bounds3, offset: usize, primitives: u32 },
  Placeholder,
}
impl BVHNode {
  pub fn bounds(&self) -> Bounds3 {
    match self {
      BVHNode::Interior { bounds, .. } => *bounds,
      BVHNode::Leaf { bounds, .. } => *bounds,
      BVHNode::Placeholder => panic!("BVH Constructed Incorrectly"),
    }
  }
}

enum BuildNode {
  Interior {
    bounds: Bounds3,
    children: [Arc<BuildNode>; 2],
    split_axis: u8,
  },
  Leaf {
    bounds: Bounds3,
    first_primitive: usize,
    primitive_count: usize,
  }
}

struct PrimitiveInfo {
  index: usize,
  bounds: Bounds3,
  centroid: Point3,
}

impl BuildNode {
  pub fn bounds(&self) -> Bounds3 {
    match &self {
      BuildNode::Interior { bounds, .. } => *bounds,
      BuildNode::Leaf { bounds, .. } => *bounds,
    }
  }
  pub fn new_interior(split_axis: u8, c0: Arc<BuildNode>, c1: Arc<BuildNode>) -> BuildNode {
    let bounds = c0.bounds().union(&c1.bounds());
    BuildNode::Interior {
      bounds,
      children: [c0, c1],
      split_axis,
    }
  }
  pub fn new_leaf(first_primitive: usize, primitive_count: usize, bounds: Bounds3) -> BuildNode {
    BuildNode::Leaf {
      bounds,
      first_primitive,
      primitive_count,
    }
  }
}

impl BVHAggregate {
  pub fn new(primitives: Vec<PrimitiveInstance>, max_node_size: usize, split_method: SplitMethod) -> BVHAggregate {
    let mut primitives = primitives;
    if primitives.len() == 0 {
      return BVHAggregate { max_node_size, split_method, primitives, nodes: vec![] }
    }

    // Compute the bounds and centroid for each primitive
    let primitive_count = primitives.len();
    let mut primitive_info = Vec::with_capacity(primitive_count);
    for (index, primitive) in primitives.iter().enumerate() {
      let bounds = primitive.world_bounds();
      let centroid = bounds.min * 0.5 + bounds.max * 0.5;
      primitive_info.push(PrimitiveInfo { index, bounds, centroid });
    }

    let mut ordered_primitives = vec![];
    // TODO: bump allocate
    // let mut arena = Bump::new();
    let (root, count) = match split_method {
      SplitMethod::Linear => unimplemented!("HLBVH Not Implemented Yet"),
      _ => Self::recursive_build(&mut primitives, &mut primitive_info, 0..primitive_count, &mut ordered_primitives, &split_method),
    };
    
    // Now, compactify the tree for efficient traversal
    let mut nodes = Vec::with_capacity(count);
    Self::flatten_build_tree(&root, &mut nodes);

    BVHAggregate {
      max_node_size,
      nodes,
      primitives: ordered_primitives,
      split_method
    }
  }

  fn recursive_build(
    primitives: &mut Vec<PrimitiveInstance>,
    primitive_info: &mut Vec<PrimitiveInfo>,
    range: Range<usize>,
    ordered_primitives: &mut Vec<PrimitiveInstance>,
    split_method: &SplitMethod,
  ) -> (BuildNode, usize) {

    let primitive_count = range.end - range.start;
    if primitive_count == 1 {
      let first_primitive = ordered_primitives.len();
      // Take ownership of this primitive
      let primitive = std::mem::replace(&mut primitives[range.start], NullPrimitive {}.into());
      let bounds = primitive.world_bounds();
      ordered_primitives.push(primitive);
      return (BuildNode::new_leaf(first_primitive, 1, bounds), 1);
    }

    // Compute the bounds for our whole range of primitives
    let mut bounds = primitives[range.start].world_bounds();
    for primitive in &primitives[range.start+1..range.end] {
      bounds = bounds.union(&primitive.world_bounds());
    }
    
    // Project the centroid of each primitive onto each axis, to choose an axis to partition on
    let mut centroid_bounds = Bounds3::new(primitive_info[range.start].centroid, primitive_info[range.start].centroid);
    for primitive in &primitive_info[range.start+1..range.end] {
      centroid_bounds = centroid_bounds.encompass(primitive.centroid);
    }
    let split_axis = centroid_bounds.maximum_dimension();

    // If we've partitioned to the point where the centroids of the bounding boxes are all at the same point,
    // return a leaf
    if centroid_bounds.max[split_axis] == centroid_bounds.min[split_axis] {
      let first_primitive = ordered_primitives.len();
      for idx in range {
        let primitive = std::mem::replace(&mut primitives[idx], NullPrimitive {}.into());
        ordered_primitives.push(primitive);
      }
      return (BuildNode::new_leaf(first_primitive, primitive_count, bounds), 1);
    }

    // otherwise, partition according to the split method
    let mut mid: usize = (range.start + range.end) / 2;
    if matches!(split_method, SplitMethod::Middle) {
      let split_point: f64 = (centroid_bounds.min[split_axis] + centroid_bounds.max[split_axis]) / 2.;
      mid = range.start + partition_index(
        &mut primitive_info.as_mut_slice()[range.clone()],
        |p| p.centroid[split_axis] < split_point
      );
    }
    let finished = mid != range.start && mid != range.end;
    if matches!(split_method, SplitMethod::EqualCounts) || !finished {
      mid = (range.start + range.end) / 2;
      primitive_info.as_mut_slice()[range.clone()]
        .select_nth_unstable_by(
          mid,
          |a, b| a.centroid[split_axis].partial_cmp(&b.centroid[split_axis]).unwrap()
        );
    }

    let (left, left_count) = Self::recursive_build(primitives, primitive_info, range.start..mid, ordered_primitives, split_method);
    let (right, right_count) = Self::recursive_build(primitives, primitive_info, mid..range.end, ordered_primitives, split_method);

    return (BuildNode::new_interior(split_axis, Arc::new(left), Arc::new(right)), left_count + right_count + 1);
  }

  fn flatten_build_tree(
    root: &BuildNode,
    nodes: &mut Vec<BVHNode>,
  ) {
    match &root {
      BuildNode::Leaf { bounds, first_primitive, primitive_count } => {
        nodes.push(BVHNode::Leaf {
          bounds: *bounds,
          offset: *first_primitive,
          primitives: *primitive_count as u32,
        });
      },
      BuildNode::Interior { bounds, children, split_axis } => {
        // Reserve our spot in the array
        let idx = nodes.len();
        nodes.push(BVHNode::Placeholder);
        Self::flatten_build_tree(children[0].borrow(), nodes);
        let second_child = nodes.len() as u32;
        Self::flatten_build_tree(children[1].borrow(), nodes);
        let _ = std::mem::replace(&mut nodes[idx], BVHNode::Interior {
          bounds: *bounds,
          axis: *split_axis,
          second_child
        });
      }
    }
  }
}

impl Primitive for BVHAggregate {
  fn world_bounds(&self) -> Bounds3<f64> {
    self.nodes[0].bounds()
  }

  fn intersect(&self, ray: &Ray) -> Option<Interaction> {
    let inv_dir = Vector3::new(1. / ray.direction.x, 1. / ray.direction.y, 1. / ray.direction.z);
    let is_neg = [
      if inv_dir.x < 0. { 1 } else { 0 },
      if inv_dir.y < 0. { 1 } else { 0 },
      if inv_dir.z < 0. { 1 } else { 0 },
    ];

    let mut visit_offset = 0;
    let mut curr_node = 0;
    let mut nodes_to_visit = [0; 64];

    let mut found_interaction = None;
    let mut ray = ray.clone();
    loop {
      let node = &self.nodes[curr_node];
      if node.bounds().any_intersect_precomputed(&ray, inv_dir, is_neg) {
        match node {
          BVHNode::Interior { axis, second_child, .. } => {
            let (next, later) = if is_neg[*axis as usize] == 1 {
              (*second_child as usize, curr_node + 1)
            } else {
              (curr_node + 1, *second_child as usize)
            };
            curr_node = next;
            nodes_to_visit[visit_offset] = later;
            visit_offset += 1;
          },
          BVHNode::Leaf { offset, primitives, .. } => {
            let start = *offset;
            let end = offset + *primitives as usize;
            // Check the ray against each of the primitives in this leaf node
            for primitive in &self.primitives[start..end] {
              if let Some(interaction) = primitive.intersect(&ray) {
                ray.time_max = interaction.intersection.distance;
                found_interaction = Some(interaction);
              }
            }
            if visit_offset == 0 {
              break;
            }
            visit_offset -= 1;
            curr_node = nodes_to_visit[visit_offset];
          },
          BVHNode::Placeholder => panic!("Improperly constructed BVH Tree"),
        }
      } else {
        if visit_offset == 0 {
          break;
        }

        visit_offset -= 1;
        curr_node = nodes_to_visit[visit_offset];
      }
    }
    return found_interaction;
  }
}

// TODO: contribute to llogiq/partition
fn partition_index<T, P>(data: &mut [T], predicate: P) -> usize
where P: Fn(&T) -> bool {
    let len = data.len();
    if len == 0 { return 0; }
    let (mut l, mut r) = (0, len - 1);
    loop {
        while l < len && predicate(&data[l]) { l += 1; }
        while r > 0 && !predicate(&data[r]) { r -= 1; }
        if l >= r { return l; }
        data.swap(l, r);
    }
}