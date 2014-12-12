#[deriving(Show)]
pub struct Process<'p> {
    pub next: Option<Box<Process<'p>>>,
    pub argv: &'p [&'p str],
    pub pid: uint,
    pub completed: bool,
    pub stopped: bool,
    pub status: int
}
