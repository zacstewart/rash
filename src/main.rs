extern crate rustbox;

use job::Job;
use std::io::{Command, stdio, IoError};

mod job;
mod process;


fn main() {
    let jobs: Vec<Job> = vec!();

    let mut stdin = stdio::stdin();
    let mut stdout = stdio::stdout();

    loop {
        stdout.write_str("[rash] $ ");
        stdout.flush();
        let line = stdin.read_line().unwrap();
        let line = line.trim();
        let mut argv = line.split_str(" ").collect::<Vec<&str>>();
        let program = argv.remove(0).unwrap();
        let mut process = match Command::new(program).args(argv.as_slice()).env("PWD", "/").spawn() {
            Ok(p) => p,
            Err(e) => panic!("Failed execution: {}", e)
        };
        let output = match process.stdout.as_mut().unwrap().read_to_end() {
            Ok(o) => o,
            Err(e) => panic!("Failed to get output: {}", e)
        };
        stdout.write(output.as_slice());
        stdout.flush();
    }
}
