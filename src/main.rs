use std::{env, io::Read, process, error::Error};

use easy_config::Config;

fn main() {
    let args = env::args();
    let config: Result<Config, Box<dyn Error>>;

    if args.len() > 1 {
        config = Config::from_args(args);
    } else {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        config = Config::from_string(&buffer);
    }

    let config_result = config.unwrap_or_else(|error| {
        eprintln!("\x1b[31mProblem passing arguments: {error}");
        process::exit(1);
    });

    if let Err(error) = easy_config::run(config_result) {
        eprintln!("\x1b[31mApplication error: {}", error);
        process::exit(1);
    }
}
