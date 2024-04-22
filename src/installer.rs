use std::{error::Error, process::Command, thread::{self, JoinHandle}, time::Duration};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
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
    let libraries: Vec<&LibraryConfig> = data.library
        .iter()
        .filter(|library| !library.allow_async.unwrap_or(false))
        .collect();
    let async_libraries: Vec<&LibraryConfig> = data.library
        .iter()
        .filter(|library| library.allow_async.unwrap_or(false))
        .collect();

    spawn_runner(async_libraries);
    install_libraries(libraries);

    Ok(())
}

fn spawn_runner(libraries: Vec<&LibraryConfig>) {
    let mut thread_handles: Vec<JoinHandle<()>> = Vec::new();
    
    for library in libraries {
        let library_data = library.clone();
        let handle = thread::spawn(|| {
            let bar = ProgressBar::new_spinner();
            bar.enable_steady_tick(Duration::from_millis(100));
            bar.set_style(
                ProgressStyle::with_template("{spinner} {wide_msg} \x1b[33m[{elapsed}]")
                .unwrap()
            );
            bar.set_message(format!("\x1b[0mRunning: \x1b[32m{}", library_data.name));
            install_library(library_data);
            bar.finish();
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }
}

fn install_libraries(libraries: Vec<&LibraryConfig>) {
    let bar = ProgressBar::new(libraries.len().try_into().unwrap_or(1));
    bar.enable_steady_tick(Duration::from_millis(100));
    bar.set_style(
        ProgressStyle::with_template("[{pos}/{len}] {spinner} {wide_msg} \x1b[33m[{elapsed}]\n{wide_bar:40.cyan/blue}")
            .unwrap()
    );

    for library in libraries {
        bar.set_position(bar.position() + 1);
        bar.set_message(format!("Running: \x1b[32m{}", library.name));
        install_library(library.clone());
    }
    bar.finish();
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
