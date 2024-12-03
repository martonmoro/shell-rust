#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::HashSet, process};

fn main() {
    loop {
        let mut builtins = HashSet::new();
        builtins.insert("exit");
        builtins.insert("echo");
        builtins.insert("type");

        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input_trimmed = input.trim();

        let (command, arguments) = match input_trimmed.split_once(' ') {
            Some((command, args)) => (command, args),
            None => (input_trimmed, ""),
        };

        match command {
            "exit" if arguments == "0" => process::exit(0),
            "echo" => println!("{}", arguments),
            "type" => {
                if builtins.contains(arguments) {
                    println!("{} is a shell builtin", arguments);
                } else {
                    println!("{}: not found", arguments)
                }
            }
            _ => {
                // Invalid command
                println!("{}: command not found", command);
                io::stdout().flush().unwrap();
            }
        }
    }
}
