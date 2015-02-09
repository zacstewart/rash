#![feature(box_syntax)]
use std::io::{stdio};
use job::Job;

mod job;
mod process;
mod pipe;

struct Shell;

impl<'s> Shell {
    fn new() -> Shell {
        Shell
    }

    fn start(&mut self) {
        loop {
            let mut stdin = stdio::stdin();
            let mut stdout = stdio::stdout();
            stdout.write_str("[rush] $ ");
            stdout.flush();

            match stdin.read_line() {
                Ok(line) => {
                    let line = line.as_slice();
                    let mut job = Job::new(line);
                    job.launch()
                },
                Err(e) => { }
            }
        }
    }
}

fn main() {
    let mut shell = Shell::new();
    shell.start();
}
