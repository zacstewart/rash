extern crate rustbox;

mod process;

fn main() {
    let p = process::Process {
        next: None,
        argv: &["this", "is", "some", "args"],
        pid: 1,
        completed: false,
        stopped: false,
        status: 0
    };
    println!("{}", p);
}
