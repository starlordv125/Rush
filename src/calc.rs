// This is my first rust program

// For exit()
use std::process::exit;
//use std::process::Command;
// For user input
use text_io::read;

pub fn main() {
    loop {
        display_menu();
        let function: i32 = read!();
        if function == 0 {
            println!("Exiting...");
            break;
        } else if function >= 5 || function <=-1 {
            println!("Error: Invalid integer entered");
            println!("Exiting...");
            break;
        }
        print!("Enter first number: ");
        let x: f64 = read!();
        print!("Enter second number: ");
        let y: f64 = read!();
        let answer = math(x, y, function);
        match answer.0 {
            "" => break,
            _ => ()
        };
        println!("{x} {} {y} = {}", answer.0, answer.1);
        print!("Contiue?\n(Y or N): ");
        let choice: char = read!();
        match choice {
            'N' | 'n' => {
                println!("Exiting...");
                break;
            },
            'Y' | 'y' => (),
            _ => {
                println!("Error: wrong value entered\nExiting...");
                break; //change this to loop
            }
        };
    }
}

fn math(x: f64, y: f64, function: i32) -> (&'static str, f64) {
    let answer: (&'static str, f64) = match function {
        1 => ("+", x + y),
        2 => ("-", x - y),
        3 => ("*", x * y),
        4 => match y {
            0.0 => {
                println!("Error: cannot divide by zero");
                println!("Exiting...");
                ("", 0.0)
            }
            _ => ("/", x / y)
            },
        _ => {
            println!("Critical Error: Valid integer not entered.");
            println!("Aborting...");
            exit(3);
        }
    };
    answer
}

fn display_menu() {
    print!("
***SIMPLE RUST CALCULATOR***
************v0.3************
1. Add
2. Subtract
3. Multiply
4. Divide
(1-4, 0 to exit): ")
}