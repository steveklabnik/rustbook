//! Implementation of the `build` subcommand, used to compile a book.

use std::os;
use std::io;
use std::io::{fs, Command, File, BufferedWriter, TempDir, IoResult};

use subcommand::Subcommand;
use term::Term;
use error::{Error, CliResult};
use book;
use book::{Book, BookItem};
use css;

use regex::Regex;

struct Build;

pub fn parse_cmd(name: &str) -> Option<Box<Subcommand>> {
    if name == "build" {
        Some(box Build as Box<Subcommand>)
    } else {
        None
    }
}

fn write_toc(book: &Book, path_to_root: &Path, out: &mut Writer) -> IoResult<()> {
    fn walk_items(items: &[BookItem], section: &str, path_to_root: &Path, out: &mut Writer) -> IoResult<()> {
        for (i, item) in items.iter().enumerate() {
            try!(walk_item(item, format!("{}{}.", section, i + 1).as_slice(), path_to_root, out));
        }
        Ok(())
    }
    fn walk_item(item: &BookItem, section: &str, path_to_root: &Path, out: &mut Writer) -> IoResult<()> {
        try!(writeln!(out, "<li><a href='{}'><b>{}</b> {}</a>",
                 path_to_root.join(item.path.with_extension("html")).display(),
                 section,
                 item.title));
        if !item.children.is_empty() {
            try!(writeln!(out, "<ul class='section'>"));
            walk_items(item.children.as_slice(), section, path_to_root, out);
            try!(writeln!(out, "</ul>"));
        }
        try!(writeln!(out, "</li>"));

        Ok(())
    }

    try!(writeln!(out, "<div id='toc'>"));
    try!(writeln!(out, "<ul class='chapter'>"));
    try!(walk_items(book.chapters.as_slice(), "", path_to_root, out));
    try!(writeln!(out, "</ul>"));
    try!(writeln!(out, "</div>"));

    Ok(())
}

fn render(book: &Book, tgt: &Path) -> CliResult<()> {
    let tmp = TempDir::new("rust-book")
                      .expect("could not create temporary directory"); // FIXME: lift to Result instead

    for (section, item) in book.iter() {
        println!("{} {}", section, item.title);

        let md_urls = regex!(r"\[(?P<title>[^]]*)\]\((?P<url_stem>[^)]*)\.(?P<ext>md|markdown)\)");

        // preprocess the markdown, rerouting markdown references to html references
        let markdown_data = try!(File::open(&item.path).read_to_string());
        let preprocessed_path = tmp.path().join(item.path.filename().unwrap());
        {
            try!(File::create(&preprocessed_path)
                      .write_str(md_urls.replace_all(markdown_data.as_slice(),
                                                     "[$title]($url_stem.html)").as_slice()));
        }

        // write the prelude to a temporary HTML file for rustdoc inclusion
        let prelude = tmp.path().join("prelude.html");
        {
            let mut toc = BufferedWriter::new(try!(File::create(&prelude)));
            write_toc(book, &item.path_to_root, &mut toc);
            try!(writeln!(toc, "<div id='page-wrapper'>"));
            try!(writeln!(toc, "<div id='page'>"));
        }

        // write the postlude to a temporary HTML file for rustdoc inclusion
        let postlude = tmp.path().join("postlude.html");
        {
            let mut toc = BufferedWriter::new(try!(File::create(&postlude)));
            try!(writeln!(toc, "</div></div>"));
        }

        let out_path = tgt.join(item.path.dirname());
        try!(fs::mkdir_recursive(&out_path, io::UserDir));

        let output_result = Command::new("rustdoc")
            .arg(&preprocessed_path)
            .arg("-o").arg(&out_path)
            .arg(format!("--html-before-content={}", prelude.display()))
            .arg(format!("--html-after-content={}", postlude.display()))
            .arg("--markdown-css").arg(item.path_to_root.join("rust-book.css"))
            .arg("--markdown-no-toc")
            .output();
        match output_result {
            Ok(output) => {
                if !output.output.is_empty() || !output.error.is_empty() {
                    return Err(box format!("{}\n{}",
                                           String::from_utf8_lossy(output.output.as_slice()),
                                           String::from_utf8_lossy(output.error.as_slice()))
                               as Box<Error>);
                }
            }
            Err(e) => {
                return Err(box format!("Could not execute `rustdoc`: {}", e) as Box<Error>);
            }
        }
    }

    Ok(())
}

impl Subcommand for Build {
    fn parse_args(&mut self, args: &[String]) -> CliResult<()> {
        Ok(())
    }
    fn usage(&self) {}
    fn execute(self, term: &mut Term) {
        let cwd = os::getcwd();
        let src = cwd.clone();
        let tgt = cwd.join("_book");

        fs::mkdir(&tgt, io::UserDir); // FIXME: handle errors

        File::create(&tgt.join("rust-book.css")).write_str(css::STYLE); // FIXME: handle errors

        let summary = File::open(&src.join("SUMMARY.md"));
        match book::parse_summary(summary, &src) {
            Ok(book) => {
                // execute rustdoc on the whole book
                render(&book, &tgt).map_err(|err| {
                    term.err(format!("error: {}", err.description()).as_slice());
                    err.detail().map(|detail| {
                        term.err(format!("detail: {}", detail).as_slice());
                    })
                });
            }
            Err(errors) => {
                for err in errors.move_iter() {
                    term.err(err.as_slice());
                }
            }
        }
    }
}
