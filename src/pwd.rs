use std::{env, path::PathBuf};

pub fn cwd() {
    // Gets current directory and assigns unreadable format to variable
    let currentwd: PathBuf = env::current_dir().expect("Error: Could not get current working directory");
    // .display() is to turn the file path into a human-readable format
    println!("The current directory is {}", currentwd.display());
}