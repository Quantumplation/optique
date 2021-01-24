#![feature(try_blocks, clamp)]

mod geometry;
mod options;
mod render;
mod scene;
use std::{path::PathBuf, sync::Arc, unimplemented};

use clap::Clap;
use geometry::{Bounds2, Point2, Point3, Transform, Vector3};
use options::*;
use render::*;
use scene::{AreaLight, GeometricPrimitive, LightInstance, PointLight, PrimitiveInstance, PrimitiveList, Scene, ShapeInstance, SphereShape};

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
    
    let s1 = Transform::translate(Vector3 { x: 0., y: 0., z: -7. });
    let s2 = Transform::translate(Vector3 { x: 0., y: 0., z: 0. });

    let scene = Scene::new(
        PrimitiveInstance::from(
            PrimitiveList {
                primitives: vec![
                    PrimitiveInstance::from(GeometricPrimitive {
                        shape: ShapeInstance::from(SphereShape { object_to_world: s1, radius: 4. }),
                        emission: Some(AreaLight { emitted_color: Spectrum::default() })
                    }),
                    PrimitiveInstance::from(GeometricPrimitive {
                        shape: ShapeInstance::from(SphereShape { object_to_world: s2, radius: 1. }),
                        emission: None
                    }),
                ]
            }
        ),
        vec![
            LightInstance::from(PointLight { position: Point3 { x: 1., y: 5., z: 5. }, color: Spectrum { r: 25., g: 25., b: 25. } })
        ],
    );

    let cam_trans = Transform::look_at(
        Point3 { x: 5., y: 5.0, z: 3. },
        Point3 { x: 0., y: 0., z: -2.5 },
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
    i.render(&scene);
    i.camera.film().write_to(options.out_file.unwrap_or(PathBuf::from("./out.png")));
}
