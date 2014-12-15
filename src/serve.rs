//! Implementation of the `serve` subcommand. Just a stub for now.

use subcommand::Subcommand;
use error::CliResult;
use term::Term;

struct Serve;

pub fn parse_cmd(name: &str) -> Option<Box<Subcommand>> {
    if name == "serve" {
        Some(box Serve as Box<Subcommand>)
    } else {
        None
    }
}

impl Subcommand for Serve {
    fn parse_args(&mut self, _: &[String]) -> CliResult<()> {
        Ok(())
    }
    fn usage(&self) {}
    fn execute(&mut self, _: &mut Term) {}
}
