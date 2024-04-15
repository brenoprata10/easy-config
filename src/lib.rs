use std::{env, fs, error::Error};

use serde::Deserialize;

pub struct Config {
    file_path: String,
    supress_errors: bool
}

#[derive(Deserialize, Debug)]
pub struct Data {
    library: Vec<LibraryConfig>,
}

#[derive(Deserialize, Debug)]
pub struct LibraryConfig {
    name: String,
    pre_install_script: String,
    install_script: String,
    post_install_script: String,
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let toml_string = fs::read_to_string(config.file_path)?;
    let library_config: Data = toml::from_str(&toml_string)?;

    println!("{:?}",library_config);

    Ok(())
}
