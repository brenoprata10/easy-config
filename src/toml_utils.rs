use std::{fs, error::Error};
use crate::installer::Data;

pub fn read_file(path: String) -> Result<Data, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    serialize_data(&content)
}

pub fn serialize_data(content: &String) -> Result<Data, Box<dyn Error>> {
    let library_config: Data = toml::from_str(content)?;

    Ok(library_config)
}
