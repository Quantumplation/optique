use std::{num::ParseIntError, path::PathBuf, str::FromStr};

use clap::Clap;

#[derive(Default, Debug)]
pub struct Rect {
  min_x: u32,
  max_x: u32,
  min_y: u32,
  max_y: u32,
}

impl FromStr for Rect {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");
        let result: Result<Rect, ()> = try {
          Rect {
            min_x: parts.next().ok_or(())?.parse::<u32>().or(Err(()))?,
            max_x: parts.next().ok_or(())?.parse::<u32>().or(Err(()))?,
            min_y: parts.next().ok_or(())?.parse::<u32>().or(Err(()))?,
            max_y: parts.next().ok_or(())?.parse::<u32>().or(Err(()))?,
          }
        };
        result.or(Err("Invalid rect.  Expected format: x0,x1,y0,y1".to_string()))
    }
}

pub enum LogLevel {
  Info,
  Warning,
  Error,
  Fatal
}
impl FromStr for LogLevel {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "0" | "info" => Ok(LogLevel::Info),
      "1" | "warn" | "warning" => Ok(LogLevel::Warning),
      "2" | "error" => Ok(LogLevel::Error),
      "3" | "fatal" => Ok(LogLevel::Fatal),
      _ => Err("Unrecognized log level".to_string()),
    }
  }
}

/// Command Line Physically-Based Renderer, derived from http://www.pbr-book.org/
#[derive(Clap)]
#[clap(version="0.1", author = "Pi Lanningham <pi.lanningham@gmail.com>")]
pub struct Options {
  /// Specify an image crop window.
  #[clap(long, value_name="x0,x1,y0,y1")]
  pub crop_window: Option<Rect>,
  /// Use specified number of threads for rendering.
  #[clap(long = "nthreads")]
  pub threads: Option<u32>,
  /// Write the final image to the given filename.
  #[clap(long = "outfile")]
  pub out_file: Option<PathBuf>,
  /// Automatically reduce a number of quality settings to render more quickly.
  #[clap(long)]
  pub quick: bool,
  /// Supress all text output other than error messages.
  #[clap(long)]
  pub quiet: bool,
  /// Specify directory that log files should be written to (defaults to system temp directory).
  #[clap(long = "logdir")]
  pub log_directory: Option<PathBuf>,
  /// Print all logging messages to stderr.
  #[clap(long = "logtostderr")]
  pub log_to_stderr: bool,
  /// Log messages at or above this level (info, warning, error, or fatal)
  #[clap(long = "minloglevel", default_value = "info")]
  pub min_log_level: LogLevel,
  #[clap(long, parse(from_occurrences))]
  pub verbosity: u32,
  /// Print a reformatted version of the input file(s) to standard output.  Does not render an image.
  #[clap(long)]
  pub cat: bool,
  /// Print a reformatted verison of the input file(s) to standard output, and convert all triangle meshes to PLY files.  Does not render an image.
  #[clap(long)]
  pub toply: bool,
  /// Input pbrt files to render
  pub input_files: Vec<PathBuf>,
}