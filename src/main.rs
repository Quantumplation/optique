#![feature(try_blocks, clamp, partition_point)]

mod geometry;
mod options;
mod render;
mod scene;
mod ply;
use std::{path::PathBuf, sync::Arc, time::Instant, unimplemented};

use clap::Clap;
use geometry::{Bounds2, Normal3, Point2, Point3, Transform, Vector3};
use options::*;
use ply::read_ply;
use render::*;
use scene::{AreaLight, BVHAggregate, BVHNode, DiskShape, GeometricPrimitive, LightInstance, Matte, PointLight, PrimitiveInstance, PrimitiveList, Scene, ShapeInstance, SphereShape, SplitMethod, TriangleMesh};

fn main() {
    let options: Options = Options::parse();

    let (_scene, _state) = if true || options.input_files.len() == 1 {
        let mut scene_info = pbrt_rs::Scene::default();
        let mut state = pbrt_rs::State::default();
        pbrt_rs::read_pbrt_file(
            "scenes/killeroo-simple/killeroo-simple.pbrt",
            &mut scene_info,
            &mut state,
        );
        (scene_info, state)
    } else if options.input_files.len() == 0 {
        unimplemented!("Reading from stdin is currently unimplemented!");
    } else {
        unimplemented!("Reading multiple files is currently unimplemented!");
    };
    
    let s0 =
        Transform::translate(Vector3 { x: 0., y: -1., z: -5. }) *
        Transform::scale(Vector3 { x: 20., y: 20., z: 20. });
    let s1 = Transform::translate(Vector3 { x: 3.75, y: 0., z: -7. });
    let s2 = Transform::translate(Vector3 { x: 1.25, y: 0., z: -7. });
    let s3 = Transform::translate(Vector3 { x: -1.25, y: 0., z: -7. });
    let s4 = Transform::translate(Vector3 { x: -3.75, y: 0., z: -7. });
    let s5 =
        Transform::translate(Vector3::new(0., -1., -5.)) *
        Transform::rotate(90., Vector3::new(1., 0., 0.));

    let mesh = Arc::new(read_ply(s0, "scenes/bunny/bun.ply".into()));
    let tris = mesh.to_triangles();
    let mut prims: Vec<PrimitiveInstance> = tris.into_iter().map(|t| GeometricPrimitive {
        shape: t,
        emission: None,
        material: Some(Matte { color: Spectrum { r: 0.5, g: 0.7, b: 0.7 }, roughness: 0. }.into()),
    }.into()).collect();

    prims.append(&mut vec![
        GeometricPrimitive {
            shape: DiskShape { object_to_world: s5, height: 0., radius: 20., inner_radius: 1.}.into(),
            material: Some(Matte { color: Spectrum { r: 0.8, g: 0.3, b: 0.2 }, roughness: 0. }.into()),
            emission: None,
        }.into(),
        GeometricPrimitive {
            shape: SphereShape { object_to_world: s1, radius: 1. }.into(),
            material: Some(Matte { color: Spectrum { r: 0.576, g: 0.859, b: 0.475 }, roughness: 0. }.into()),
            emission: None,
        }.into(),
        GeometricPrimitive {
            shape: SphereShape { object_to_world: s2, radius: 1. }.into(),
            material: Some(Matte { color: Spectrum { r: 0.576, g: 0.859, b: 0.475 }, roughness: 20. }.into()),
            emission: None
        }.into(),
        GeometricPrimitive {
            shape: SphereShape { object_to_world: s3, radius: 1. }.into(),
            material: Some(Matte { color: Spectrum { r: 0.576, g: 0.859, b: 0.475 }, roughness: 50. }.into()),
            emission: None
        }.into(),
        GeometricPrimitive {
            shape: SphereShape { object_to_world: s4, radius: 1. }.into(),
            material: Some(Matte { color: Spectrum { r: 0.576, g: 0.859, b: 0.475 }, roughness: 80. }.into()),
            emission: None
        }.into(),
    ]);

    let bvh = BVHAggregate::new(prims, 100, SplitMethod::Middle);

    let scene = Scene::new(
        bvh.into(),
        vec![
            LightInstance::from(PointLight { position: Point3 { x: -2., y: 5., z: 3. }, color: Spectrum { r: 200., g: 200., b: 200. } }),
        ],
    );

    let cam_trans = Transform::look_at(
        Point3 { x: 0., y: 5.0, z: 5. },
        Point3 { x: 0., y: 0., z: -7. },
        Vector3 { x: 0., y: 1., z: 0. }
    ).inverse();
    let mut i = WhittedIntegrator::new(
        20,
        CameraInstance::from(PerspectiveCamera::new(
            cam_trans, Bounds2 { min: Point2 { x: -0.5, y: -0.5 }, max: Point2 { x: 0.5, y: 0.5 } },
            0., 0., 0., 0., 75.,
            Arc::new(Film::new(Point2 { x: 500, y: 500 }))
        )),
        SamplerInstance::from(NullSampler {}),
    );
        
    println!("Starting...");
    let start = Instant::now();

    i.render(&scene);

    println!("Finished.  Took: {:.2}s", start.elapsed().as_secs_f64());

    i.camera.film().write_to(options.out_file.unwrap_or(PathBuf::from("./out.png")));
}
