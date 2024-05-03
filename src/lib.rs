use std::error::Error;

use installer::LibraryConfig;

use crate::installer::Data;

mod toml_utils;
mod installer;

pub struct Config {
    data: Data,
    queries: Option<Vec<String>>
}

impl Config {
    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Config, Box<dyn Error>> {
        args.next();

        let data = match args.next() {
            Some(arg) => toml_utils::read_file(arg)?,
            None => return Err("Could not fetch file path.".into())
        };

        let mut queries: Vec<String> = Vec::new();

        loop {
            let arg = args.next();
            if arg.is_none() {
                break;
            }
            queries.push(arg.unwrap());
        }

        Ok(Config {data, queries: Some(queries)})
    }

    pub fn from_string(content: &String) -> Result<Config, Box<dyn Error>> {
        let data = toml_utils::serialize_data(&content)?;

        Ok(Config {data, queries: None})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let queried_data: Vec<LibraryConfig> = config.data.library.into_iter().filter(|library| {
        if config.queries.is_none() {
            return true;
        }

        match &library.id {
            Some(id) => config.queries
                .clone()
                .is_some_and(|queries| queries
                     .iter()
                     .filter(|query| query.to_uppercase() == id.to_uppercase())
                     .count() > 0
                ),
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

    if queried_data.is_empty() && config.queries.is_some() {
        return Err(
            format!(
                "\x1b[0mCould not find any libraries with given ids: \x1b[33m{:?}", 
                config.queries
            ).into()
        );
    }

    installer::install(queried_data)?;

    Ok(())
}
