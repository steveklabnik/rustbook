//! An abstraction of the terminal. Eventually, provide color and
//! verbosity support. For now, just a wrapper around stdout/stderr.

use std::io::stdio;

pub struct Term {
    err: Box<Writer + 'static>
}

impl Term {
    pub fn new() -> Term {
        Term {
            err: box stdio::stderr() as Box<Writer>,
        }
    }

    pub fn err(&mut self, msg: &str) {
        // swallow any errors
        let _ = self.err.write_line(msg);
    }
}
