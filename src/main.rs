#![feature(try_blocks)]

mod options;
mod scene;
mod geometry;
mod render;
use std::unimplemented;

use clap::Clap;
use options::*;

fn main() {
    let options: Options = Options::parse();

    let scene = if options.input_files.len() == 1 {
        let mut scene_info = pbrt_rs::Scene::default();
        let mut state = pbrt_rs::State::default();
        pbrt_rs::read_pbrt_file(options.input_files[0].to_str().unwrap(), &mut scene_info, &mut state);
        (scene_info, state)
    } else if options.input_files.len() == 0 {
        unimplemented!("Reading from stdin is currently unimplemented!");
    } else {
        unimplemented!("Reading multiple files is currently unimplemented!");
    };
    
    println!("Image size: {:?}", scene.0.image_size);
    println!("Objects: {}", scene.0.objects.len());
    println!("Shapes: {}", scene.0.shapes.len());
    println!("Lights: {}", scene.0.lights.len());
    println!("Materials: {}", scene.0.materials.len());
    println!("Instances: {}", scene.0.instances.len());
}
