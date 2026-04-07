// Terminal-based file explorer made in Rust
// Cameron Reynolds;
// NO AI USED

mod calc;

//LIBRARIES GO HERE, DESCRIBE THEM!!
use text_io::read;
// For listing contents of directories
use std::env;
use std::path::{PathBuf};

// External library, not sure what for yet
//use anyhow::Result;
use std::env::consts::OS;
use std::process::exit;

// For cd
use std::path::Path;

// Keep main() clean, don't repeat code
fn main() {
    println!("***FILE EXPLORER V0.1-alpha***");
    shell();
    //println!("{}", test)
}

// Not entirely sure what Result<()> means yet
fn cwd() {
    // Gets current directory and assigns unreadable format to variable
    let currentwd: PathBuf = env::current_dir().expect("REASON");
    // .display() is to turn the file path into a human-readable format
    println!("The current directory is {}", currentwd.display());
}

fn user_input(opsys: &str) -> String {
    match opsys {
        "linux" => read!("{}\n"),
        "windows" => read!("{}\r\n"),
        _ => {
            println!("Error: OS not supported");
            exit(2)
        }
    }
}

fn get_os(opsys: &str) -> (&'static str, PathBuf, &'static str) {
    let currentwd: PathBuf = env::current_dir().expect("REASON");
    match opsys {
        "linux" => ("", currentwd, "$ "),
        "windows" => ("RS ", currentwd, "> "),
        _ => {
            println!("Critical Error: OS not supported");
            panic!("Program is running on unsupported OS")
        }
    }
}

fn change_dir() {
    print!("Directory to change to: ");
    let input_dir: String = read!();
    let root = Path::new(&input_dir);
    assert!(env::set_current_dir(root).is_ok());
}

fn shell() {
    let opsys = OS;
    loop {
        let osinfo = get_os(opsys);
        print!("{}{}{}", osinfo.0, osinfo.1.display(), osinfo.2);
        let command: String = user_input(opsys).replace("\n", "");
        match command.as_str() {
            "" => continue,
            "cd" => change_dir(),
            "pwd" => cwd(),
            "calc" => {
                calc::main();
            }
            "help" => {
                println!("Rust Shell v0.1-alpha");
                println!("pwd -> Prints current working directory");
                println!("calc -> Opens calculator application");
                println!("help -> Displays this menu");
                println!("exit -> Exits the program");
            }
            "exit" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Rush: Command not found...");
                println!("Type \"help\" for list of commands");
            }
        };
    }
}