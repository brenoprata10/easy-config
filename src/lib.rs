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

        while let Some(arg) = args.next() {
            queries.push(arg);
        }

        Ok(Config {data, queries: Some(queries)})
    }

    pub fn from_string(content: &String) -> Result<Config, Box<dyn Error>> {
        let data = toml_utils::serialize_data(&content)?;

        Ok(Config {data, queries: None})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let queries = config.queries.unwrap_or(vec![]);
    let has_empty_queries = queries.is_empty();
    let library_data: Vec<LibraryConfig> = config.data.library
        .into_iter()
        .filter(|library| {
            if has_empty_queries {
                return true;
            }

            match &library.id {
                Some(id) => queries
                    .iter()
                    .any(|query| query.to_uppercase() == id.to_uppercase()),
                None => false
            }
        }).collect();

    let has_sudo_commands = library_data
        .iter()
        .any(|library| library.install_script.contains("sudo"));

    if has_sudo_commands {
        // Make user type password to avoid blocking the sudo processes
        installer::runner("sudo ls")?;
    }

    if library_data.is_empty() && !has_empty_queries {
        return Err(
            format!(
                "\x1b[0mCould not find any libraries with given ids: \x1b[33m{:?}", 
                queries
            ).into()
        );
    }

    installer::install(library_data)?;

    Ok(())
}
