use std::{process::Command, error::Error};
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
    let error_output = "Command failed";

    for library in data.library {
        eprintln!("\x1b[36m=======================================\n");
        eprintln!("\x1b[0mInstalling: \x1b[32m{}", library.name);

        let output = runner(&library.install_script).unwrap_or_else(|error| {
            let error_message = String::from(
                format!("{}: {}\n{}", library.name, library.install_script, error)
            );
            eprintln!("\x1b[31m{error_message}");
            error_output.to_string()
        });

        eprintln!("\x1b[0m{output}\n");

        if output != error_output {
            println!("\x1b[32m{} Installed Successfully!\n", library.name);
        }
    }

    Ok(())
}

fn runner(command: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(&command)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr.into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    Ok(stdout.to_string())
}
