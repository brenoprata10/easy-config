use std::{error::Error, process::Command, thread::{self, JoinHandle}, time::Duration};
use indicatif::ProgressBar;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    library: Vec<LibraryConfig>,
}

#[derive(Deserialize, Clone)]
pub struct LibraryConfig {
    name: String,
    install_script: String,
    allow_async: Option<bool>,
}

pub fn install(data: Data) -> Result<(), Box<dyn Error>> {
    let libraries: Vec<&LibraryConfig> = data.library.iter().filter(|library| !library.allow_async.unwrap_or(false)).collect();
    let async_libraries: Vec<&LibraryConfig> = data.library.iter().filter(|library| library.allow_async.unwrap_or(false)).collect();
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(100));
    bar.finish();

    spawn_runner(async_libraries);

    for library in libraries {
        install_library(library.clone());
    }

    Ok(())
}

fn spawn_runner(libraries: Vec<&LibraryConfig>) {
    let mut thread_handles: Vec<JoinHandle<()>> = Vec::new();
    
    for library in libraries {
        let library_data = library.clone();
        let handle = thread::spawn(|| {
            install_library(library_data);
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
}


fn install_library(library: LibraryConfig) {
    for command in library.install_script.split("&&") {
        runner(command).unwrap_or_else(|error| {
            let error_message = String::from(
                format!("{}: {}\n{}", library.name, library.install_script, error)
                );
            eprintln!("\x1b[31m{error_message}");
            "Command failed".to_string()
        });
    }
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
