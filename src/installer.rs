use std::{error::Error, process::Command, thread::{self, JoinHandle}, time::Duration};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

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
    let multi_progress_bar = Arc::new(Mutex::new(MultiProgress::new()));

    let thread_handles = spawn_runner(async_libraries, &multi_progress_bar);
    install_libraries(libraries, &multi_progress_bar);

    for handle in thread_handles {
        handle.join().unwrap();
    }

    Ok(())
}

fn spawn_runner(libraries: Vec<&LibraryConfig>, multi_progress_bar: &Arc<Mutex<MultiProgress>>) -> Vec<JoinHandle<()>> {
    let mut thread_handles: Vec<JoinHandle<()>> = Vec::new();
    
    for library in libraries {
        let multi_progress_clone = Arc::clone(multi_progress_bar);
        let library_data = library.clone();
        let handle = thread::spawn(move || {
            let library_name = library_data.name.clone();
            let added_bar = multi_progress_clone.lock().unwrap().add(ProgressBar::new_spinner());
            added_bar.enable_steady_tick(Duration::from_millis(100));
            added_bar.set_style(
                ProgressStyle::with_template("{spinner} \x1b[0mRunning: \x1b[32m{wide_msg}\t")
                    .unwrap()
            );
            added_bar.set_message(library_name.clone());
            added_bar.finish();

            match install_library(library_data) {
                Ok(()) => added_bar.set_message(format!("\x1b[32m✓ {}", added_bar.message())),
                Err(error) => added_bar.set_message(
                    format!("\x1b[31m✗ {} failed. \n\n  {}:\n  {}", added_bar.message(), library_name, error)
                )
            }
        });
        thread_handles.push(handle);
    }

    thread_handles
}

fn install_libraries(libraries: Vec<&LibraryConfig>, multi_progress_bar: &Arc<Mutex<MultiProgress>>) {
    let mut errors: Vec<String> = Vec::new();
    let added_bar = multi_progress_bar.lock().unwrap().add(ProgressBar::new(libraries.len().try_into().unwrap_or(1)));
    added_bar.enable_steady_tick(Duration::from_millis(100));
    added_bar.set_style(
        ProgressStyle::with_template("{spinner} \x1b[0mRunning: \x1b[33m[{pos}/{len}] \x1b[0m- \x1b[32m{wide_msg}\t")
            .unwrap()
    );

    for library in libraries {
        added_bar.set_position(added_bar.position() + 1);
        added_bar.set_message(library.name.clone());
        if let Err(error) = install_library(library.clone()) {
            errors.push(format!("\n  {}: \n  {}", library.name, error.to_string()));
        }
    }

    if errors.len() == 0 {
        added_bar.set_message("\x1b[32m✓ Completed");
    } else {
        added_bar.set_message(format!("\x1b[31m✗ {} operation(s) failed.\n", errors.len()));
        errors.iter().for_each(|error| {
            added_bar.set_message(format!("{}{}", added_bar.message(), error));
        });
    }
    added_bar.finish();
}


fn install_library(library: LibraryConfig) -> Result<(), Box<dyn Error>> {
    for command in library.install_script.split("&&") {
        runner(command)?;
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
