#![feature(try_blocks)]

mod geometry;
mod options;
mod render;
mod scene;
use std::{path::PathBuf, sync::Arc, unimplemented};

use clap::Clap;
use geometry::{Point2, Point3};
use options::*;
use render::*;
use scene::{Scene};

fn main() {
    let options: Options = Options::parse();

    let (scene, _state) = if true || options.input_files.len() == 1 {
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

    let scene = Scene::from(&scene);

    let mut i = WhittedIntegrator::new(
        1,
        CameraInstance::from(PerspectiveCamera {
            position: Point3 { x: 0., y: 0., z: 0. },
            film: Arc::new(Film::new(Point2 { x: 100, y: 100 }))
        }),
        SamplerInstance::from(NullSampler {}),
    );
    i.render(&scene);
    i.camera.film().write_to(options.out_file.unwrap_or(PathBuf::from("./out.png")));
}
