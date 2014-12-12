use process::Process;

#[deriving(Show)]
pub struct Job<'j> {
    pub process: Process<'j>,
    pub pgid: uint,
    pub notified: bool,
    pub stdin: int,
    pub stdout: int,
    pub stderr: int
}
