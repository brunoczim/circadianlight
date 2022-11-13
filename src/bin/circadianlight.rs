#![doc = include_str!("../../README.md")]

use std::process;

use circadianlight::{cli::Program, environment::with_os_graphical_env};
use structopt::StructOpt;

fn main() {
    if let Err(error) = with_os_graphical_env(Program::from_args()) {
        eprintln!("{}", error);
        process::exit(-1);
    }
}
