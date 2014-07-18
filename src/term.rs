//! An abstraction of the terminal. Eventually, provide color and
//! verbosity support. For now, just a wrapper around stdout/stderr.

use std::io::stdio;
use std::io::IoResult;

pub struct Term {
    verbose: bool,
    out: Box<Writer>,
    err: Box<Writer>
}

impl Term {
    pub fn new() -> Term {
        Term {
            verbose: false,
            out: box stdio::stdout() as Box<Writer>,
            err: box stdio::stderr() as Box<Writer>,
        }
    }

    pub fn out(&mut self, msg: &str) {
        // swallow any errors
        let _ = self.out.write_line(msg);
    }

    pub fn err(&mut self, msg: &str) {
        // swallow any errors
        let _ = self.err.write_line(msg);
    }
}
