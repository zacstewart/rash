use crate::process::Process;
use crate::pipe::Pipe;

pub struct Job<'j> {
    processes: Vec<Process<'j>>,
}

impl<'j> Job<'j> {
    pub fn new(line: &'j str) -> Job<'j> {
        let processes = line.split('|').map(|command|
            Process::new(command, Pipe::from_stdin(), Pipe::to_stdout())
        ).collect();
        Job { processes }
    }

    pub fn launch(&mut self) {
        for process in self.processes.iter_mut() {
            process.launch();
        }
    }
}
