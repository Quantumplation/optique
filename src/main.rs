#![feature(try_blocks)]

mod geometry;
mod options;
mod render;
mod scene;
use std::unimplemented;

use clap::Clap;
use options::*;
use render::*;
use scene::{NullPrimitive, PrimitiveInstance, Scene};

fn main() {
    let options: Options = Options::parse();

    let (scene, state) = if options.input_files.len() == 1 {
        let mut scene_info = pbrt_rs::Scene::default();
        let mut state = pbrt_rs::State::default();
        pbrt_rs::read_pbrt_file(
            options.input_files[0].to_str().unwrap(),
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
        CameraInstance::from(NullCamera {}),
        SamplerInstance::from(NullSampler {}),
    );
    i.render(&scene);
}
