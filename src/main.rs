use std::{env, io::Read, process, error::Error};

use easy_config::Config;

fn main() {
    let args = env::args();
    let config: Result<Config, Box<dyn Error>>;

    if args.len() > 1 {
        config = Config::from_file(args);
    } else {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        config = Config::from_string(&buffer);
    }

    if let Err(error) = easy_config::run(config.unwrap_or_else(|error| {
            eprintln!("Problem passing arguments: {error}");
            process::exit(1);
        })) {
        eprintln!("Application error: {}", error);
        process::exit(1);
    }
}
