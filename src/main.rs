/*
 * Copyright (c) 2025 spekie
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * 3. Neither the name of the copyright holder nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
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
