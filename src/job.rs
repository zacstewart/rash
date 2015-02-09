use process::Process;
use pipe::Pipe;

pub struct Job<'j> {
    processes: Vec<Process<'j>>
}

impl<'j> Job<'j> {
    pub fn new(line: &'j str) -> Job<'j> {
        let processes = line.split_str("|").map(|command|
            Process::new(command, Pipe::from_stdin(), Pipe::to_stdout())
        ).collect();
        Job { processes: processes }
    }

    pub fn launch(&mut self) {
        for process in self.processes.iter_mut() {
            process.launch();
        }
    }
}
