use std::{
    env,
    io::{stdin, stdout, Write},
    path::{Path, PathBuf},
    process::Command,
};

// TODO:
// 1. add pipe operation
// 2. interupt a signal but not interrupt program (run 'cat' and press ctrl-c)
// 3. add command history to shell
fn main() {
    loop {
        // Prompt
        print!("> ");
        // ensure it prints before calling read_line
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // take arguments from certain commands
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                // default to '/' as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => return,
            command => {
                let result = find_executable_in_path(command);
                match result {
                    Some(executable_path) => {
                        let result = Command::new(executable_path).args(args).spawn();
                        match result {
                            Ok(mut child) => {
                                child.wait().unwrap();
                            }
                            Err(e) => {
                                eprintln!("{}", e);
                            }
                        }
                    }
                    None => {
                        eprintln!("Command not found: {}", command);
                    }
                }
            }
        }
    }
}

fn find_executable_in_path(command: &str) -> Option<PathBuf> {
    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            let full_path = path.join(command);
            if full_path.is_file() {
                return Some(full_path);
            }
        }
    }
    None
}
