/*
 * Copyright (C) 2025 spekie
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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
