use std::error::Error;

use installer::LibraryConfig;

use crate::installer::Data;

mod toml_utils;
mod installer;

pub struct Config {
    data: Data,
    query: Option<String>
}

impl Config {
    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Config, Box<dyn Error>> {
        args.next();

         let data = match args.next() {
            Some(arg) => toml_utils::read_file(arg)?,
            None => return Err("Could not fetch file path.".into())
        };

         let query = args.next();

         Ok(Config {data, query})
    }

    pub fn from_string(content: &String) -> Result<Config, Box<dyn Error>> {
        let data = toml_utils::serialize_data(&content)?;

        Ok(Config {data, query: None})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let queried_data: Vec<LibraryConfig> = config.data.library.into_iter().filter(|library| {
        if config.query.is_none() {
            return true;
        }

        match &library.id {
            Some(id) => config.query
                .clone()
                .is_some_and(|query| query.to_uppercase() == id.to_uppercase()),
            None => false
        }
    }).collect();

    let has_sudo_commands = queried_data
        .iter()
        .any(|library| library.install_script.contains("sudo"));

    if has_sudo_commands {
        // Make user type password to avoid blocking the sudo processes
        installer::runner("sudo ls")?;
    }

    if queried_data.is_empty() && config.query.is_some() {
        return Err(
            format!(
                "\x1b[0mCould not find any libraries with given id: \x1b[33m{}", 
                config.query.unwrap_or(".".to_string())
            ).into()
        );
    }

    installer::install(queried_data)?;

    Ok(())
}
