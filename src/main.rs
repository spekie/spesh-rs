use std::io::{self, Write};
use std::process::Command;

fn execute_command(command: &str) {
    let args = command.split_whitespace().collect::<Vec<&str>>();

    if let Some(command) = args.get(0) {
        let status = Command::new(command)
            .args(&args[1..])
            .status();

        match status {
            Ok(status) => {
                if !status.success() {
                    eprintln!("Command failed with status: {}", status);
                }
            }
            Err(e) => {
                eprintln!("Failed to execute command: {}", e);
            }
        }
    }
}

fn main() {
    println!("spesh, the Simple Program Execution SHell\n");

    loop {
        print!("spesh >$ ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut command = String::new();

        if io::stdin().read_line(&mut command).is_err() {
            eprintln!("Input error");
            continue;
        }

        let command = command.trim();

        if command == "exit" {
            break;
        }

        execute_command(command);
    }
}
