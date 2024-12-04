#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env,
    path::{Path, PathBuf},
    process::{self, Command},
};

fn find_exec(name: &str) -> Option<PathBuf> {
    if let Ok(paths) = env::var("PATH") {
        for path in env::split_paths(&paths) {
            let exec_path = path.join(name);
            if exec_path.is_file() {
                return Some(exec_path);
            }
        }
    }
    None
}

fn main() {
    loop {
        let builtins = ["exit", "echo", "type", "pwd", "cd"];

        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input_trimmed = input.trim();

        let argv = input_trimmed.split_whitespace().collect::<Vec<&str>>();
        let cmd = argv[0];
        let args = &argv[1..];
        if builtins.contains(&cmd) {
            match cmd {
                "exit" => process::exit(0),
                "echo" => println!("{}", args.join(" ")),
                "type" => {
                    if args.len() != 1 {
                        println!("type: expected 1 argument, got {}", args.len());
                    }
                    let type_cmd = args[0];
                    if builtins.contains(&type_cmd) {
                        println!("{} is a shell builtin", type_cmd);
                    } else {
                        match find_exec(type_cmd) {
                            Some(exec_path) => {
                                if let Some(path_str) = exec_path.to_str() {
                                    println!("{} is {}", type_cmd, path_str);
                                } else {
                                    println!("Error: Path contains invalid Unicode");
                                }
                            }
                            None => println!("{}: not found", type_cmd),
                        }
                    }
                }
                "pwd" => match env::current_dir() {
                    Ok(curr_dir) => {
                        println!("{}", curr_dir.display())
                    }
                    Err(e) => eprintln!("error getting working directory: {e}"),
                },
                "cd" => {
                    if args.len() != 1 {
                        println!("type: expected 1 argument, got {}", args.len());
                    }
                    let mut path = args[0].to_string();
                    if args[0].starts_with("~") {
                        let home = env::var("HOME").unwrap();
                        path = path.replace("~", &home);
                    }
                    match std::env::set_current_dir(Path::new(&path)) {
                        Ok(_) => (),
                        Err(_) => eprintln!("cd: {}: No such file or directory", path),
                    }
                }
                _ => unreachable!(),
            }
        } else if let Some(path) = find_exec(cmd) {
            Command::new(path)
                .args(args)
                .status() // the new process inherits the terminal's streams
                .expect("failed to execute");
        } else {
            // Invalid command
            println!("{}: command not found", input_trimmed);
            io::stdout().flush().unwrap();
        }
    }
}
