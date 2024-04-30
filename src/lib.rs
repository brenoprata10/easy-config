use std::error::Error;

use crate::installer::Data;

mod toml_utils;
mod installer;

pub struct Config {
    data: Data,
}

impl Config {
    pub fn from_file(mut args: impl Iterator<Item = String>) -> Result<Config, Box<dyn Error>> {
        args.next();

        let data = match args.next() {
            Some(arg) => toml_utils::read_file(arg)?,
            None => return Err("Could not fetch file path.".into())
        };

        Ok(Config {data})
    }

    pub fn from_string(content: &String) -> Result<Config, Box<dyn Error>> {
        let data = toml_utils::serialize_data(&content)?;

        Ok(Config {data})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    installer::install(config.data)?;

    Ok(())
}
