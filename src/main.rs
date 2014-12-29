// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(slicing_syntax)]

#![feature(macro_rules)]
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
);

mod term;
mod error;
mod book;

mod subcommand;
mod help;
mod build;
mod serve;
mod test;

mod css;

#[cfg(not(test))] // thanks #12327
fn main() {
    let mut term = Term::new();
    let cmd = os::args();

    match cmd.tail().head() {
        Some(name) => {
            match subcommand::parse_name(name[]) {
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
