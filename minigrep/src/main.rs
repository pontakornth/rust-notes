use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::{run, Config};

fn main() {
    let args = env::args();

    // 0 is the program name.
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
