//! Implementation of the `test` subcommand. Just a stub for now.

use subcommand::Subcommand;
use error::CliResult;
use term::Term;

struct Test;

pub fn parse_cmd(name: &str) -> Option<Box<Subcommand>> {
    if name == "test" {
        Some(box Test as Box<Subcommand>)
    } else {
        None
    }
}

impl Subcommand for Test {
    fn parse_args(&mut self, _: &[String]) -> CliResult<()> {
        Ok(())
    }
    fn usage(&self) {}
    fn execute(&mut self, _: &mut Term) {}
}
