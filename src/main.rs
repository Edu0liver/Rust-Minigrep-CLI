#![allow(unused_variables, dead_code)]

use std::env;
use std::process;

mod minigrep;
use crate::minigrep::minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args)
    .unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments!");
        process::exit(1)
    });

    match run(config) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Application Error!");
            process::exit(1)
        }
    }

}
