#![feature(try_blocks)]

mod options;
use clap::Clap;
use options::*;

fn main() {
    let options: Options = Options::parse();
}
