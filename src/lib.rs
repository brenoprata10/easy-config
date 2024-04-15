use std::env;

#[derive(Debug)]
pub struct Config {
    file_path: String,
    supress_errors: bool
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Could not fetch file path.")
        };

        let supress_errors = env::var("SUPRESS_ERRORS").is_ok();

        Ok(Config {file_path, supress_errors})
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    Ok(())
}
