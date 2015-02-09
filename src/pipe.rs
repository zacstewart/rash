use std::old_io::stdio::StdinReader;
use std::old_io::util::NullWriter;
use std::old_io::{Reader,Writer,ChanReader,ChanWriter,IoResult};
use std::old_io::{stdio};
use std::sync::mpsc::channel;

pub struct Pipe {
    reader: Option<ChanReader>,
    writer: Option<ChanWriter>
}

impl Pipe {
    pub fn new() -> Pipe {
        let (tx, rx) = channel();
        Pipe {
            reader: Some(ChanReader::new(rx)),
            writer: Some(ChanWriter::new(tx))
        }
    }

    pub fn from_stdin() -> Pipe {
        let (tx, rx) = channel();
        Pipe {
            reader: None,
            writer: Some(ChanWriter::new(tx))
        }
    }

    pub fn to_stdout() -> Pipe {
        let (tx, rx) = channel();
        Pipe {
            reader: Some(ChanReader::new(rx)),
            writer: None
        }
    }
}

impl Writer for Pipe {
    fn write_all(&mut self, buf: &[u8]) -> IoResult<()> {
        match self.writer {
            Some(ref mut w) => w.write_all(buf),
            None => stdio::stdout().write_all(buf)
        }
    }
}

impl Reader for Pipe {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        match self.reader {
            Some(ref mut r) => r.read(buf),
            None => Ok(0) //stdio::stdin().read(buf)
        }
    }
}
