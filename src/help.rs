//! Implementation of the `help` subcommand. Currently just prints basic usage info.

use subcommand::Subcommand;
use error::CliResult;
use term::Term;

struct Help;

pub fn parse_cmd(name: &str) -> Option<Box<Subcommand>> {
    match name {
        "help" | "--help" | "-h" | "-?" => Some(box Help as Box<Subcommand>),
        _ => None
    }
}

impl Subcommand for Help {
    fn parse_args(&mut self, _: &[String]) -> CliResult<()> {
        Ok(())
    }
    fn usage(&self) {}
    fn execute(&mut self, _: &mut Term) {
        usage()
    }
}

pub fn usage() {
    println!("Usage: rust-book <command> [<args>]");
    println!("");
    println!("The <command> must be one of:");
    println!("  help    Print this message.");
    println!("  build   Build the book in subdirectory _book");
    println!("  serve   --NOT YET IMPLEMENTED--");
    println!("  test    --NOT YET IMPLEMENTED--");
}
