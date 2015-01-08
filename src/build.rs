// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation of the `build` subcommand, used to compile a book.

use std::os;
use std::io;
use std::io::{fs, Command, File, BufferedWriter, TempDir, IoResult};

use subcommand::Subcommand;
use term::Term;
use error::{Error, CliResult, CommandResult};
use book;
use book::{Book, BookItem};
use css;

use regex::Regex;

use rustdoc;

struct Build;

pub fn parse_cmd(name: &str) -> Option<Box<Subcommand>> {
    if name == "build" {
        Some(box Build as Box<Subcommand>)
    } else {
        None
    }
}

fn write_toc(book: &Book, path_to_root: &Path, out: &mut Writer) -> IoResult<()> {
    fn walk_items(items: &[BookItem],
                  section: &str,
                  path_to_root: &Path,
                  out: &mut Writer) -> IoResult<()> {
        for (i, item) in items.iter().enumerate() {
            try!(walk_item(item, &format!("{}{}.", section, i + 1)[], path_to_root, out));
        }
        Ok(())
    }
    fn walk_item(item: &BookItem,
                 section: &str,
                 path_to_root: &Path,
                 out: &mut Writer) -> IoResult<()> {
        try!(writeln!(out, "<li><a href='{}'><b>{}</b> {}</a>",
                 path_to_root.join(item.path.with_extension("html")).display(),
                 section,
                 item.title));
        if !item.children.is_empty() {
            try!(writeln!(out, "<ul class='section'>"));
            let _ = walk_items(&item.children[], section, path_to_root, out);
            try!(writeln!(out, "</ul>"));
        }
        try!(writeln!(out, "</li>"));

        Ok(())
    }

    try!(writeln!(out, "<div id='toc'>"));
    try!(writeln!(out, "<ul class='chapter'>"));
    try!(walk_items(&book.chapters[], "", path_to_root, out));
    try!(writeln!(out, "</ul>"));
    try!(writeln!(out, "</div>"));

    Ok(())
}

fn render(book: &Book, tgt: &Path) -> CliResult<()> {
    let tmp = TempDir::new("rust-book")
                      .ok()
                      // FIXME: lift to Result instead
                      .expect("could not create temporary directory");

    for (section, item) in book.iter() {
        println!("{} {}", section, item.title);

        let out_path = tgt.join(item.path.dirname());

        let regex = r"\[(?P<title>[^]]*)\]\((?P<url_stem>[^)]*)\.(?P<ext>md|markdown)\)";
        let md_urls = Regex::new(regex).unwrap();

        let src;
        if os::args().len() < 3 {
            src = os::getcwd().unwrap().clone();
        } else {
            src = Path::new(os::args()[2].clone());
        }
        // preprocess the markdown, rerouting markdown references to html references
        let markdown_data = try!(File::open(&src.join(&item.path)).read_to_string());
        let preprocessed_path = tmp.path().join(item.path.filename().unwrap());
        {
            let urls = md_urls.replace_all(&markdown_data[], "[$title]($url_stem.html)");
            try!(File::create(&preprocessed_path)
                      .write_str(&urls[]));
        }

        // write the prelude to a temporary HTML file for rustdoc inclusion
        let prelude = tmp.path().join("prelude.html");
        {
            let mut toc = BufferedWriter::new(try!(File::create(&prelude)));
            let _ = write_toc(book, &item.path_to_root, &mut toc);
            try!(writeln!(&mut toc, "<div id='page-wrapper'>"));
            try!(writeln!(&mut toc, "<div id='page'>"));
        }

        // write the postlude to a temporary HTML file for rustdoc inclusion
        let postlude = tmp.path().join("postlude.html");
        {
            let mut toc = BufferedWriter::new(try!(File::create(&postlude)));
            try!(writeln!(&mut toc, "</div></div>"));
        }

        try!(fs::mkdir_recursive(&out_path, io::USER_DIR));

        let rustdoc_args: &[String] = &[
            preprocessed_path.display().to_string(),
            format!("-o {}", out_path.display()),
            format!("--html-before-content={}", prelude.display()),
            format!("--html-after-content={}", postlude.display()),
            format!("--markdown-css {}", item.path_to_root.join("rust-book.css").display()),
            "--markdown-no-toc".to_string(),
        ];
        let output_result = rustdoc::main_args(rustdoc_args);
        if output_result != 0 {
                return Err(box format!("Could not execute `rustdoc`: {}", output_result) as Box<Error>);
        }
    }

    // create index.html from the root README
    try!(fs::copy(&tgt.join("README.html"), &tgt.join("index.html")));
    Ok(())
}

impl Subcommand for Build {
    fn parse_args(&mut self, _: &[String]) -> CliResult<()> {
        Ok(())
    }
    fn usage(&self) {}
    fn execute(&mut self, term: &mut Term) -> CommandResult<()> {
        let cwd = os::getcwd().unwrap();
        let src;
        let tgt;

        if os::args().len() < 3 {
            src = cwd.clone();
        } else {
            src = Path::new(os::args()[2].clone());
        }

        if os::args().len() < 4 {
            tgt = cwd.join("_book");
        } else {
            tgt = Path::new(os::args()[3].clone());
        }

        let _ = fs::mkdir(&tgt, io::USER_DIR); // FIXME: handle errors

        // FIXME: handle errors
        let _ = File::create(&tgt.join("rust-book.css")).write_str(css::STYLE);

        let summary = File::open(&src.join("SUMMARY.md"));
        match book::parse_summary(summary, &src) {
            Ok(book) => {
                // execute rustdoc on the whole book
                let _ = render(&book, &tgt).map_err(|err| {
                    term.err(&format!("error: {}", err.description())[]);
                    err.detail().map(|detail| {
                        term.err(&format!("detail: {}", detail)[]);
                    })
                });
            }
            Err(errors) => {
                for err in errors.into_iter() {
                    term.err(&err[]);
                }
            }
        }

        Ok(()) // lol
    }
}
