use std::io::Command;
use std::io::process::ProcessExit;

#[deriving(Show)]
pub struct Process<'p, R: Reader, W: Writer> {
    pub command: &'p str,
    pub arguments: Box<[&'p str]>,
    pub pid: usize,
    pub completed: bool,
    pub stopped: bool,
    stdin: R,
    stdout: W
}

impl<'p, R: Reader, W: Writer> Process<'p, R, W> {
    pub fn new(line: &'p str, reader: R, mut writer: W) -> Process<'p, R, W> {
        let mut argv = line.split_str(" ").collect::<Vec<&str>>();
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

        let input = self.stdin.read_to_end().unwrap();
        let input = input.as_slice().clone();
        process.stdin.as_mut().unwrap().write(input);

        let output = process.stdout.as_mut().unwrap().read_to_end().unwrap();
        let output = output.as_slice().clone();
        self.stdout.write(output);
        self.stdout.flush();

        match process.wait() {
            Ok(status) => return status,
            Err(error) => panic!("Couldn't wait")
        }
    }
}
