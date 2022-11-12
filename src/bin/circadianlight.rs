use std::process;

use circadianlight::{cli::Program, environment::with_os_graphical_env};

fn main() {
    if let Err(error) = with_os_graphical_env(Program::parse()) {
        eprintln!("{}", error);
        process::exit(-1);
    }
}
