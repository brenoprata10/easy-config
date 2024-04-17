use std::error::Error;

mod toml_utils;
mod installer;

pub struct Config {
    file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Could not fetch file path.")
        };

        Ok(Config {file_path})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let data = toml_utils::read_file(config)?;
    installer::install(data)?;

    Ok(())
}
