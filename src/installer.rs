use std::{error::Error, io::{self, BufRead, BufReader}, process::{ChildStdout, Command, Stdio}};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    library: Vec<LibraryConfig>,
}

#[derive(Deserialize)]
pub struct LibraryConfig {
    name: String,
    install_script: String,
}

pub fn install(data: Data) -> Result<(), Box<dyn Error>> {
    for library in data.library {
        println!("\n\x1b[33m=======================================\n");
        println!("\x1b[0mInstalling: \x1b[32m{}", library.name);

        for command in library.install_script.split("&&") {
            print!("\x1b[0mRunning: \x1b[32m{}", command.trim());

            let output = runner(command);

            match output {
                Ok(stdout) => {
                    // Forcing blue color, adding the color in the foreach will not color the font
                    // for some commands
                    println!("\x1b[36m");
                    stdout
                        .lines()
                        .filter_map(|line| line.ok())
                        .for_each(|line| println!("\n{line}\n"));
                }
                Err(error) => {
                    let error_message = String::from(
                        format!("{}: {}\n{}", library.name, library.install_script, error)
                        );
                    eprintln!("\x1b[31m{error_message}");
                }
            }
        }
    }

    Ok(())
}

fn runner(command: &str) -> Result<BufReader<ChildStdout>, Box<dyn Error>> {
    let stdout = Command::new("/bin/sh")
        .arg("-c")
        .arg(&command)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other,"Could not capture standard output."))?;

    let reader = BufReader::new(stdout);

    Ok(reader)
}
