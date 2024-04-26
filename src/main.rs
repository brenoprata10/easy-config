use std::{env, process};

use easy_config::Config;

enum RunningAs {
    Root,
    User,
    Suid,
}


fn main() {
    match get_user_privileges() {
        RunningAs::Root => println!("\n\x1b[0mEasy Config is running as 'sudo'.\n"),
        _ => println!("\n\x1b[33mEasy Config is not running as 'sudo'\nThe program will freeze if any of your libraries use sudo privileges.\n")
    }

    let config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem passing arguments: {error}");
        process::exit(1);
    });

    if let Err(error) = easy_config::run(config) {
        eprintln!("Application error: {}", error);
        process::exit(1);
    }
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
