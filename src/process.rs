use std::io::Command;
use std::io::process::ProcessExit;
use pipe::Pipe;

pub struct Process<'p> {
    pub command: &'p str,
    pub arguments: Box<[&'p str]>,
    pub pid: usize,
    pub completed: bool,
    pub stopped: bool,
    stdin: Pipe,
    stdout: Pipe
}

impl<'p> Process<'p> {
    pub fn new(line: &'p str, reader: Pipe, writer: Pipe) -> Process<'p> {
        let mut argv = line.trim().split_str(" ").collect::<Vec<&str>>();
        let command = argv.remove(0);
        let arguments = argv.into_boxed_slice();

        Process {
            command: command,
            arguments: arguments,
            pid: 0,
            completed: false,
            stopped: false,
            stdin: reader,
            stdout: writer
        }
    }

    pub fn launch(&mut self) -> ProcessExit {
        let box ref args = self.arguments;
        let mut process = match Command::new(self.command)
            .args(args)
            .spawn() {
                Ok(p) => p,
                Err(e) => panic!("Failed execution: {}", e)
            };

        match self.stdin.read_to_end() {
            Ok(input) => {
                let input = input.as_slice().clone();
                match process.stdin.as_mut() {
                    Some(stdin) => stdin.write(input),
                    _ => Ok(())
                };
            },
            Err(e) => {}
        }

        match process.stdout.as_mut() {
            Some(stdout) => match stdout.read_to_end() {
                Ok(output) => {
                    let output = output.as_slice().clone();
                    self.stdout.write(output);
                },
                Err(e) => {}
            },
            None => {}
        }

        self.stdout.flush();

        match process.wait() {
            Ok(status) => return status,
            Err(error) => panic!("Couldn't wait")
        }
    }
}
