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

fn split_with_quotes(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_word = String::new();
    let mut chars = input.chars().peekable();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;

    while let Some(c) = chars.next() {
        match c {
            // Handle backslashes
            '\\' => {
                if let Some(&next_char) = chars.peek() {
                    // In double quotes, only certain characters are escaped
                    if in_double_quotes {
                        match next_char {
                            '\\' | '$' | '"' | '\n' => {
                                chars.next(); // Consume the escaped character
                                current_word.push(next_char);
                            }
                            _ => {
                                current_word.push('\\');
                                current_word.push(next_char);
                                chars.next();
                            }
                        }
                    } else if in_single_quotes {
                        current_word.push('\\');
                        current_word.push(next_char);
                        chars.next();
                    } else {
                        // Outside quotes, preserve the literal value of the next character
                        chars.next(); // Consume the escaped character
                        current_word.push(next_char);
                    }
                }
            }
            // Handle quotes
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
            }
            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes;
            }
            // Handle spaces
            ' ' if !in_single_quotes && !in_double_quotes => {
                if !current_word.is_empty() {
                    result.push(current_word);
                    current_word = String::new();
                }
            }
            // Handle all other characters
            _ => {
                current_word.push(c);
            }
        }
    }

    // Add the last word if there is one
    if !current_word.is_empty() {
        result.push(current_word);
    }

    result
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
        if input_trimmed.is_empty() {
            continue;
        }

        let argv = split_with_quotes(input_trimmed);
        if argv.is_empty() {
            continue;
        }

        let cmd = &argv[0];
        let args = &argv[1..];

        if builtins.contains(&cmd.as_str()) {
            match cmd.as_str() {
                "exit" => process::exit(0),
                "echo" => println!("{}", args.join(" ")),
                "type" => {
                    if args.len() != 1 {
                        println!("type: expected 1 argument, got {}", args.len());
                        continue;
                    }
                    let type_cmd = &args[0];
                    if builtins.contains(&type_cmd.as_str()) {
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
                    Ok(curr_dir) => println!("{}", curr_dir.display()),
                    Err(e) => eprintln!("error getting working directory: {e}"),
                },
                "cd" => {
                    if args.len() != 1 {
                        println!("cd: expected 1 argument, got {}", args.len());
                        continue;
                    }
                    let mut path = args[0].to_string();
                    if path.starts_with('~') {
                        if let Ok(home) = env::var("HOME") {
                            path = path.replace('~', &home);
                        }
                    }
                    if let Err(_) = std::env::set_current_dir(Path::new(&path)) {
                        eprintln!("cd: {}: No such file or directory", path);
                    }
                }
                _ => unreachable!(),
            }
        } else if let Some(path) = find_exec(cmd) {
            Command::new(path)
                .args(args)
                .status()
                .expect("failed to execute");
        } else {
            println!("{}: command not found", cmd);
            io::stdout().flush().unwrap();
        }
    }
}
