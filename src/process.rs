use std::io::process::ProcessExit;
use std::io::Command;

#[deriving(Show)]
pub struct Process<'p, R: Reader, W: Writer> {
    pub command: &'p str,
    pub arguments: &'p [&'p str],
    pub pid: uint,
    pub completed: bool,
    pub stopped: bool,
    stdin: R,
    stdout: W
}

impl<'p, R: Reader, W: Writer> Process<'p, R, W> {
    pub fn new(line: &str, reader: R, mut writer: W) -> Process<R, W> {
        let mut argv = line.split_str(" ").collect::<Vec<&str>>();
        let command = argv.remove(0).unwrap();
        let arguments = argv.as_slice().clone();
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
        let mut process = match Command::new(self.command)
            .args(self.arguments)
            .spawn() {
                Ok(p) => p,
                Err(e) => panic!("Failed execution: {}", e)
            };

        let input = self.stdin.read_to_end().unwrap().as_slice().clone();
        process.stdin.as_mut().unwrap().write(input);

        let output = process.stdout.as_mut().unwrap().read_to_end().unwrap().as_slice().clone();
        self.stdout.write(output);
        self.stdout.flush();

        match process.wait() {
            Ok(status) => return status,
            Err(error) => panic!("Couldn't wait")
        }
    }
}
