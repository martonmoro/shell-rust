#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, fs, process};

fn main() {
    let path = env::var("PATH").unwrap();
    loop {
        // let mut builtins = HashSet::new();
        // builtins.insert("exit");
        // builtins.insert("echo");
        // builtins.insert("type");

        let builtins = ["exit", "echo", "type"];

        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input_trimmed = input.trim();

        let argv = input_trimmed.split_whitespace().collect::<Vec<&str>>();

        match argv[0] {
            "exit" => process::exit(0),
            "echo" => println!("{}", argv[1..].join(" ")),
            "type" => {
                if argv.len() != 2 {
                    println!("type: expected 1 argument, got {}", argv.len() - 1);
                }
                let cmd = argv[1];
                if builtins.contains(&cmd) {
                    println!("{} is a shell builtin", cmd);
                } else {
                    let split = &mut path.split(":");
                    if let Some(path) =
                        split.find(|path| fs::metadata(format!("{}/{}", path, cmd)).is_ok())
                    {
                        println!("{cmd} is {path}/{cmd}")
                    } else {
                        println!("{}: not found", cmd)
                    }
                }
            }
            _ => {
                // Invalid command
                println!("{}: command not found", input_trimmed);
                io::stdout().flush().unwrap();
            }
        }
    }
}
