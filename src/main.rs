use std::{env, process};

use easy_config::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem passing arguments: {error}");
        process::exit(1);
    });

    if let Err(error) = easy_config::run(config) {
        eprintln!("Application error: {}", error);
        process::exit(1);
    }
}
