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
            let bar = ProgressBar::new_spinner();
            bar.enable_steady_tick(Duration::from_millis(100));
            bar.set_style(
                ProgressStyle::with_template("{spinner} {wide_msg} \x1b[33m[{elapsed}]")
                .unwrap()
            );
            bar.set_message(format!("\x1b[0mRunning: \x1b[32m{}", library_data.name));
            let added_bar = multi_progress_clone.lock().unwrap().add(bar);
            install_library(library_data);
            added_bar.finish();
        });
        thread_handles.push(handle);
    }

    thread_handles

}

fn install_libraries(libraries: Vec<&LibraryConfig>, multi_progress_bar: &Arc<Mutex<MultiProgress>>) {
    let bar = ProgressBar::new(libraries.len().try_into().unwrap_or(1));
    bar.enable_steady_tick(Duration::from_millis(100));
    bar.set_style(
        ProgressStyle::with_template("[{pos}/{len}] {spinner} {wide_msg} \x1b[33m[{elapsed}]\n{wide_bar:40.cyan/blue}")
            .unwrap()
    );

    let added_bar = multi_progress_bar.lock().unwrap().add(bar);

    for library in libraries {
        added_bar.set_position(added_bar.position() + 1);
        added_bar.set_message(format!("Running: \x1b[32m{}", library.name));
        install_library(library.clone());
    }

    added_bar.finish();
}


fn install_library(library: LibraryConfig) {
    for command in library.install_script.split("&&") {
        runner(command);
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
