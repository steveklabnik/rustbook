# rustbook

A simplified version of gitbook, atop rustdoc.

**Warning**: this is an early work in progress, providing the minimum
  functionality needed to build the
  [Rust guidelines](https://github.com/rust-lang/rust-guidelines).

## Acquiring and building

```
git clone https://github.com/aturon/rust-book.git
cd rust-book
cargo build
```

## Usage

Like `gitbook`, the `rustbook` tool builds a book from a number of
separate markdown files. The contents of the book are determined by a
`SUMMARY.md` file like:

```markdown
# Summary

* [Why to use WhizBang](why/README.md)
    * [First reason](why/first.md)
    * [Second reason](why/first.md)
* [How to use WhizBang](how/README.md)
    * [Installing](how/installing.md)
    * [Usage](how/usage.md)
```

The setup is intended to make it easy to browse a book directly on GitHub:

* By convention, each chapter/section with children is placed in its
own subdirectory, with a `README.md` serving as the top level of the
chapter/section.

* Any interior links to files with extension `.md` or `.markdown` will
automatically be converted to `.html`.

* Books automatically include an `Introduction` section pointing to the
`README.md` in the root directory.

To build a book, run `rustbook build` in the book's root directory,
which should contain a `SUMMARY.md` and `README.md` as just described.
Currently, the output is always placed in a `_book` subdirectory,
following `gitbook` defaults.
