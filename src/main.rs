#![feature(macro_rules)]
#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

use std::os;
use subcommand::Subcommand;
use term::Term;

macro_rules! try (
    ($expr:expr) => ({
        use error;
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(error::FromError::from_err(err))
        }
    })
)

mod term;
mod error;
mod book;

mod subcommand;
mod help;
mod build;
mod serve;
mod test;

mod css;

fn main() {
    let mut term = Term::new();
    let cmd = os::args();

    match cmd.tail().head() {
        Some(name) => {
            match subcommand::parse_name(name.as_slice()) {
                Some(mut subcmd) => {
                    match subcmd.parse_args(cmd.tail()) {
                        Ok(_) => {
                            subcmd.execute(&mut term);
                        }
                        Err(err) => {
                            println!("{}", err);
                            println!("");
                            subcmd.usage();
                        }
                    }
                }
                None => {
                    println!("Unrecognized command '{}'.", name);
                    println!("");
                    help::usage();
                }
            }
        }
        None => {
            help::usage();
        }
    }
}
