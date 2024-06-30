use std::process::{Command, ExitStatus};
use std::io::{Read, Write};
use crate::pipe::Pipe;

pub struct Process<'p> {
    pub command: &'p str,
    pub arguments: Box<[&'p str]>,
    stdin: Pipe,
    stdout: Pipe,
}

impl<'p> Process<'p> {
    pub fn new(line: &'p str, reader: Pipe, writer: Pipe) -> Process<'p> {
        let mut argv = line.trim().split_whitespace().collect::<Vec<&str>>();
        let command = argv.remove(0);
        let arguments = argv.into_boxed_slice();

        Process {
            command,
            arguments,
            stdin: reader,
            stdout: writer,
        }
    }

    pub fn launch(&mut self) -> ExitStatus {
        let args: &[&str] = &self.arguments;
        let mut process = match Command::new(self.command)
            .args(args)
            .spawn() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to execute command '{}': {}", self.command, e);
                    std::process::exit(1);
                }
            };

        let mut input = Vec::new();
        self.stdin.read_to_end(&mut input).unwrap();
        if let Some(mut stdin) = process.stdin.take() {
            stdin.write_all(&input).unwrap();
        }

        let mut output = Vec::new();
        if let Some(mut stdout) = process.stdout.take() {
            stdout.read_to_end(&mut output).unwrap();
            self.stdout.write_all(&output).unwrap();
        }

        process.wait().unwrap()
    }
}
