use std::{collections::HashSet, error::Error, process::Command, thread::{self, JoinHandle}, time::Duration};
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
    group: Option<String>,
    install_script: String,
    allow_async: Option<bool>,
}

enum ProgressBarType {
    Bar(u64),
    Spinner
}

struct InstallerConfig<'a, 'b> {
    libraries: Vec<&'a LibraryConfig>, 
    multi_progress_bar: &'b Arc<Mutex<MultiProgress>>,
    groups: Option<HashSet<String>>, 
}

impl<'a, 'b> InstallerConfig<'a, 'b> {
    fn new(
        libraries: Vec<&'a LibraryConfig>,
        multi_progress_bar: &'b Arc<Mutex<MultiProgress>>
    ) -> InstallerConfig<'a, 'b> {
        InstallerConfig {
            libraries,
            multi_progress_bar,
            groups: None,
        }
    }

    fn with_groups(
        libraries: Vec<&'a LibraryConfig>, 
        multi_progress_bar: &'b Arc<Mutex<MultiProgress>>,
        groups: HashSet<String>
    ) -> InstallerConfig<'a, 'b> {
        InstallerConfig {
            libraries,
            multi_progress_bar,
            groups: Some(groups),
        }
    }
}

struct LibraryInstallProgressBar {
    progress_bar: ProgressBar,
}

impl LibraryInstallProgressBar {
    fn new(template: &str, progress_bar_type: ProgressBarType) -> LibraryInstallProgressBar {
        let progress_bar = match progress_bar_type {
            ProgressBarType::Spinner => ProgressBar::new_spinner(),
            ProgressBarType::Bar(length) => ProgressBar::new(length)
        };
        progress_bar.set_style(ProgressStyle::with_template(template).unwrap());

        LibraryInstallProgressBar {
            progress_bar
        }
    }
}

pub fn install(data: Data) -> Result<(), Box<dyn Error>> {
    let libraries: Vec<&LibraryConfig> = data.library
        .iter()
        .filter(|library| !library.allow_async.unwrap_or(false) && library.group.is_none())
        .collect();
    let async_libraries: Vec<&LibraryConfig> = data.library
        .iter()
        .filter(|library| library.allow_async.unwrap_or(false) && library.group.is_none())
        .collect();
    let grouped_libraries: HashSet<String> = data.library
        .iter()
        .filter_map(|library| library.group.clone())
        .collect();
    let multi_progress_bar = Arc::new(Mutex::new(MultiProgress::new()));

    let async_thread_handles = install_async(
        InstallerConfig::new(async_libraries, &multi_progress_bar)
    );
    let group_thread_handles = install_groups(
        InstallerConfig::with_groups(
            data.library.iter().collect(),
            &multi_progress_bar,
            grouped_libraries, 
        )
    );

    install_libraries(
        InstallerConfig::new(libraries, &multi_progress_bar)
    );

    let thread_handles: Vec<JoinHandle<()>> = vec![
        async_thread_handles, 
        group_thread_handles
    ].into_iter().flatten().collect();

    for handle in thread_handles {
        handle.join().unwrap();
    }

    Ok(())
}

fn install_groups(config: InstallerConfig) -> Vec<JoinHandle<()>> {
    if config.groups.is_none() {
        return vec![];
    }

    let mut thread_handles: Vec<JoinHandle<()>> = Vec::new();
    let library_reference: Arc<Vec<LibraryConfig>> = Arc::new(config.libraries.into_iter().cloned().collect());

    for group in config.groups.unwrap_or(HashSet::new()) {
        let library_reference_clone = Arc::clone(&library_reference);
        let multi_progress_clone = Arc::clone(&config.multi_progress_bar);
        let group_thread_handle = thread::spawn(move || {
            let group_libraries: Vec<LibraryConfig> = library_reference_clone
                .to_vec()
                .into_iter()
                .filter(
                    |library| library.group.clone().is_some_and(|library_group| library_group == group)
                )
                .collect();
            install_libraries(
                InstallerConfig::new(
                    group_libraries.iter().collect(), 
                    &multi_progress_clone
                )
            );
        });
        thread_handles.push(group_thread_handle);
    }

    thread_handles
}

fn install_async(config: InstallerConfig) -> Vec<JoinHandle<()>> {
    let mut thread_handles: Vec<JoinHandle<()>> = Vec::new();
    
    for library in config.libraries {
        let multi_progress_clone = Arc::clone(config.multi_progress_bar);
        let library_data = library.clone();
        let handle = thread::spawn(move || {
            let library_name = library_data.name.clone();
            let library_progress_bar = LibraryInstallProgressBar::new(
                "{spinner} \x1b[0mRunning: \x1b[32m{wide_msg}\t",
                ProgressBarType::Spinner
            );
            let progress_bar = multi_progress_clone.lock().unwrap().add(
                library_progress_bar.progress_bar
            );
            progress_bar.enable_steady_tick(Duration::from_millis(100));
            progress_bar.set_message(library_name.clone());

            let install_result = install_library(library_data, &progress_bar);
            let final_message = match install_result {
                Ok(()) => format!("\x1b[32m✓ {}", progress_bar.message()),
                Err(error) => format!(
                    "\x1b[31m✗ {} failed. \n\n  {}:\n  {}",
                    progress_bar.message(), 
                    library_name, 
                    error
                )
            };

            progress_bar.set_message(final_message);
            progress_bar.finish();
        });
        thread_handles.push(handle);
    }

    thread_handles
}

fn install_libraries(config: InstallerConfig) {
    let mut errors: Vec<String> = Vec::new();
    let library_progress_bar = LibraryInstallProgressBar::new(
        "{spinner} \x1b[0mRunning: \x1b[33m[{pos}/{len}] \x1b[0m- \x1b[32m{wide_msg}\t",
        ProgressBarType::Bar(config.libraries.len() as u64)
    );
    let progress_bar = config.multi_progress_bar.lock().unwrap().add(
        library_progress_bar.progress_bar
    );
    progress_bar.enable_steady_tick(Duration::from_millis(100));

    for library in config.libraries {
        progress_bar.set_position(progress_bar.position() + 1);
        progress_bar.set_message(library.name.clone());
        if let Err(error) = install_library(library.clone(), &progress_bar) {
            errors.push(format!("\n  {}: \n  {}", library.name, error.to_string()));
        }
    }

    if errors.len() == 0 {
        progress_bar.set_message("\x1b[32m✓ Completed");
    } else {
        progress_bar.set_message(format!("\x1b[31m✗ {} operation(s) failed.\n", errors.len()));
        errors.iter().for_each(|error| {
            progress_bar.set_message(format!("{}{}", progress_bar.message(), error));
        });
    }
    progress_bar.finish();
}

fn install_library(library: LibraryConfig, progress_bar: &ProgressBar) -> Result<(), Box<dyn Error>> {
    let initial_progress_bar_message = progress_bar.message();
    for command in library.install_script.split("&&") {
        progress_bar.set_message(format!("{}\n  \x1b[36mExec  => {}", initial_progress_bar_message, command.trim()));
        runner(command)?;
    }

    progress_bar.set_message(initial_progress_bar_message);

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
