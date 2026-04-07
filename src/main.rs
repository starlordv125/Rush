// Terminal-based file explorer made in Rust
// Cameron Reynolds;
// NO AI USED

// builtins go here
mod calc;
mod pwd;
mod ls;
mod cd;

// libraries go here
use text_io::read;
use std::env;
use std::path::{PathBuf};
use colored::Colorize;
use std::env::consts::OS;
use whoami::{self, hostname};

// For cd
use std::path::Path;

// Keep main() clean, don't repeat code
fn main() {
    check_os();
    println!("***Rush V0.1-alpha***");
    shell();
}

fn check_os() {
    match OS {
        "linux" => (),
        _ => panic!("Error: OS not supported")
    }
}

// Keeping this a function for now because it will get more complex with time
fn user_input() -> String {
    read!("{}\n")
}

fn make_prompt() -> String {
    let wd: PathBuf = env::current_dir().expect("Error finding current working directory in get_os()");
    let wd = wd.display().to_string();
    let username: String = whoami::username().unwrap().to_string();
    let host = hostname().unwrap().to_string();
    return username + "@" + &host + ":" + &wd
}

fn shell() {
    loop {
        let prompt = make_prompt();
        print!("{}$ ", prompt.green());
        let command: String = user_input().replace("\n", "");
        match command.as_str() {
            "" => continue,
            "cd" => cd::change_dir(),
            "pwd" => pwd::cwd(),
            "calc" => {
                calc::main();
            }
            "help" => {
                println!("Rust Shell v0.2-alpha");
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
                println!("-rush: Command not found...");
                println!("Type \"help\" for list of commands");
            }
        };
    }
}