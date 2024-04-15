use std::{fs, error::Error};
use crate::Config;
use crate::installer::Data;


pub fn read_file(config: Config) -> Result<Data, Box<dyn Error>> {
    let toml_string = fs::read_to_string(config.file_path)?;
    let library_config: Data = toml::from_str(&toml_string)?;

    Ok(library_config)
}
