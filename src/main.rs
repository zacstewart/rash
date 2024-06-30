use std::io::{self, Write};
use job::Job;

mod job;
mod process;
mod pipe;

struct Shell;

impl Shell {
    fn new() -> Shell {
        Shell
    }

    fn start(&mut self) {
        loop {
            let mut stdout = io::stdout();
            print!("[rush] $ ");
            stdout.flush().unwrap();

            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(_) => {
                    let line = line.trim();
                    if !line.is_empty() {
                        let mut job = Job::new(line);
                        job.launch();
                    }
                },
                Err(_) => {},
            }
        }
    }
}

fn main() {
    let mut shell = Shell::new();
    shell.start();
}
