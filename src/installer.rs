use std::{cmp::Ordering, error::Error, process::Command, thread::{self, JoinHandle}};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    library: Vec<LibraryConfig>,
}

#[derive(Deserialize)]
pub struct LibraryConfig {
    name: String,
    install_script: String,
    allow_async: Option<bool>,
}

pub fn install(mut data: Data) -> Result<(), Box<dyn Error>> {
    let mut thread_handles: Vec<JoinHandle<()>> = Vec::new();

    data.library
        .sort_unstable_by(
            |lib1, lib2| {
                let lib1_allow_async = lib1.allow_async.unwrap_or(false);   
                let lib2_allow_async = lib2.allow_async.unwrap_or(false);   
                if !lib1_allow_async && lib2_allow_async
                    {Ordering::Greater} 
                else if !lib2_allow_async && lib1_allow_async
                    {Ordering::Less}
                else 
                    {Ordering::Equal}
            }
        );

    for library in data.library {
        let allow_async = library.allow_async.unwrap_or(false);
        if allow_async {
            let handle = install_library_async(library);
            thread_handles.push(handle);
        } else {
            install_library(library);
        }
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    Ok(())
}

fn install_library_async(library: LibraryConfig) -> JoinHandle<()> {
    thread::spawn(move || {
        install_library(library);
    })
}

fn install_library(library: LibraryConfig) {
    let error_output = "Command failed";
    println!("\n\x1b[33m=======================================\n");
    println!("\x1b[0mInstalling: \x1b[32m{}", library.name);

    for command in library.install_script.split("&&") {
        println!("\x1b[0mRunning: \x1b[32m{}", command.trim());

        let output = runner(command).unwrap_or_else(|error| {
            let error_message = String::from(
                format!("{}: {}\n{}", library.name, library.install_script, error)
                );
            eprintln!("\x1b[31m{error_message}");
            error_output.to_string()
        });

        println!("\x1b[0m{output}\n");
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
