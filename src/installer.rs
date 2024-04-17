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
    for library in data.library {
        println!("Installing: {}", library.name);

        let output = runner(&library.install_script).unwrap_or_else(|error| {
            let error_message = String::from(
                format!("{}: {}\n{}", library.name, library.install_script, error)
            );
            eprintln!("{error_message}");
            "Command failed.".to_string()
        });

        println!("\n{output}");
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
