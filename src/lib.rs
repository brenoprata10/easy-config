use std::{error::Error, process};

mod toml_utils;
mod installer;

enum RunningAs {
    Root,
    User,
    Suid,
}

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
    let has_sudo_commands = data.library.iter().any(|library| library.install_script.contains("sudo"));

    match get_user_privileges() {
        RunningAs::Root => (),
        _ => {
            if has_sudo_commands {
                eprint!("\n\x1b[31mFailed to execute.\n\x1b[0mYour scripts require sudo privileges.\nRerun with sudo: \x1b[33msudo easy-config <YOUR_FILE>.toml\n");
                process::exit(1);
            }
        }
    }

    installer::install(data)?;

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
