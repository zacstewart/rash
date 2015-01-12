use std::io::{stdio};
use std::io::stdio::{StdinReader, StdWriter};

mod process;

struct Shell<'s, W: Writer> {
    stdin: StdinReader,
    stdout: W
}

impl<'s, W: Writer> Shell<'s, W> {
    fn new(stdin: StdinReader, stdout: W) -> Shell<'s, W> {
        Shell {
            stdin: stdin,
            stdout: stdout
        }
    }

    fn start(&mut self) {
        loop {
            self.stdout.write_str("[rash] $ ");
            self.stdout.flush();

            let line = self.stdin.read_line().unwrap();
            let line = line.trim();
            let mut process = process::Process::new(line, stdio::stdin(), stdio::stdout());

            process.launch();
        }
    }
}

fn main() {
    let mut stdin = stdio::stdin();
    let mut stdout = stdio::stdout();
    let mut shell = Shell::new(stdin, stdout);
    shell.start();
}
