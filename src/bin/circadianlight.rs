use std::process;

use circadianlight::args::Program;

fn main() {
    if let Err(error) = Program::parse().run() {
        eprintln!("{}", error);
        process::exit(-1);
    }
}
