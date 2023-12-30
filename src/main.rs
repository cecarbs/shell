use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
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
                let result = Command::new(command).args(args).spawn();
                match result {
                    Ok(mut child) => {
                        child.wait().unwrap();
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
        }
    }
}
