use std::{env, process};

use easy_config::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem passing arguments: {error}");
        process::exit(1);
    });

    println!("{:?}", config);
}
