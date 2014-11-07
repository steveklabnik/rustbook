//! Common API for all rust-book subcommands.

use error::CliResult;
use term::Term;

use help;
use build;
use serve;
use test;

pub trait Subcommand {
    /// Mutate the subcommand by parsing its arguments.
    ///
    /// Returns `Err` on a parsing error.
    fn parse_args(&mut self, args: &[String]) -> CliResult<()>;
    /// Print the CLI usage information.
    fn usage(&self);
    /// Actually execute the subcommand.
    fn execute(&mut self, term: &mut Term);
}

/// Create a Subcommand object based on its name.
pub fn parse_name(name: &str) -> Option<Box<Subcommand>> {
    for parser in [help::parse_cmd, build::parse_cmd,
                   serve::parse_cmd, test::parse_cmd].iter() {
        let parsed = (*parser)(name);
        if parsed.is_some() { return parsed }
    }
    None
}
