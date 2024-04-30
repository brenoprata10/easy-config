use std::{error::Error, process};

use crate::installer::Data;

mod toml_utils;
mod installer;

enum RunningAs {
    Root,
    User,
    Suid,
}

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
    let has_sudo_commands = config.data.library.iter().any(|library| library.install_script.contains("sudo"));

    match get_user_privileges() {
        RunningAs::Root => (),
        _ => {
            if has_sudo_commands {
                eprint!("\n\x1b[31mFailed to execute.\n\x1b[0mYour scripts require sudo privileges.\nRerun with sudo: \x1b[33msudo easy-config <YOUR_FILE>.toml\n");
                process::exit(1);
            }
        }
    }

    installer::install(config.data)?;

    Ok(())
}

fn get_user_privileges() -> RunningAs {
    let uid = unsafe { libc::getuid() };
    let euid = unsafe { libc::geteuid() };

    match (uid, euid) {
        (0, 0) => RunningAs::Root,
        (_, 0) => RunningAs::Suid,
        (_, _) => RunningAs::User,
    }
}
