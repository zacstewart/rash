use std::io::{self, Read, Write};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct Pipe {
    reader: Option<Receiver<u8>>,
    writer: Option<Sender<u8>>,
}

impl Pipe {
    pub fn from_stdin() -> Pipe {
        let (tx, _) = channel();
        Pipe {
            reader: None,
            writer: Some(tx),
        }
    }

    pub fn to_stdout() -> Pipe {
        let (_, rx) = channel();
        Pipe {
            reader: Some(rx),
            writer: None,
        }
    }
}

impl Write for Pipe {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(ref mut w) = self.writer {
            for &byte in buf {
                w.send(byte).unwrap();
            }
            Ok(buf.len())
        } else {
            io::stdout().write(buf)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush()
    }
}

impl Read for Pipe {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(ref mut r) = self.reader {
            for (i, byte) in buf.iter_mut().enumerate() {
                match r.recv() {
                    Ok(b) => *byte = b,
                    Err(_) => return Ok(i),
                }
            }
            Ok(buf.len())
        } else {
            Ok(0)
        }
    }
}
