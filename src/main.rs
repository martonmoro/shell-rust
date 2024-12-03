#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input_trimmed = input.trim();

        match input_trimmed {
            "exit 0" => process::exit(0),
            _ => {
                // Invalid command
                println!("{}: command not found", input_trimmed);
                io::stdout().flush().unwrap();
            }
        }
    }
}
