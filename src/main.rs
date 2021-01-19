#![feature(try_blocks, clamp)]

mod geometry;
mod options;
mod render;
mod scene;
use std::{path::PathBuf, sync::Arc, unimplemented};

use clap::Clap;
use geometry::{Bounds2, Point2, Point3, Transform};
use options::*;
use render::*;
use scene::{GeometricPrimitive, LightInstance, PointLight, PrimitiveInstance, PrimitiveList, Scene, ShapeInstance, SphereShape};

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

    let scene = Scene::new(
        PrimitiveInstance::from(
            PrimitiveList {
                primitives: vec![
                    PrimitiveInstance::from(GeometricPrimitive {
                        shape: ShapeInstance::from(SphereShape { point: Point3 { x: 0., y: 0., z: -10. }, radius: 2. }),
                        emission: None
                    }),
                    PrimitiveInstance::from(GeometricPrimitive {
                        shape: ShapeInstance::from(SphereShape { point: Point3 { x: 0., y: 0., z: -10. }, radius: 2. }),
                        emission: None
                    }),
                    PrimitiveInstance::from(GeometricPrimitive {
                        shape: ShapeInstance::from(SphereShape { point: Point3 { x: 3., y: 0., z: -10. }, radius: 1. }),
                        emission: None
                    }),
                    PrimitiveInstance::from(GeometricPrimitive {
                        shape: ShapeInstance::from(SphereShape { point: Point3 { x: -3., y: -1., z: -8. }, radius: 1. }),
                        emission: None
                    }),
                ]
            }
        ),
        vec![
            LightInstance::from(PointLight { position: Point3 { x: 1., y: -2., z: -5. }, color: Spectrum { r: 10., g: 0., b: 0. } })
        ],
    );

    let mut i = WhittedIntegrator::new(
        1,
        CameraInstance::from(PerspectiveCamera::new(
            Transform::default(), Bounds2 { min: Point2 { x: -0.5, y: -0.5 }, max: Point2 { x: 0.5, y: 0.5 } },
            0., 0., 0., 0., 75.,
            Arc::new(Film::new(Point2 { x: 500, y: 500 }))
        )),
        SamplerInstance::from(NullSampler {}),
    );
    i.render(&scene);
    i.camera.film().write_to(options.out_file.unwrap_or(PathBuf::from("./out.png")));
}
