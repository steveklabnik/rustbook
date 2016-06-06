# rustbook

Build multi-page documentation with Rustdoc.

Please note this is a mirror of https://github.com/rust-lang/rust/tree/master/src/tools/rustbook , so issues should be filed on the language tracker.

[![Build Status](https://travis-ci.org/steveklabnik/rustbook.svg?branch=master)](https://travis-ci.org/steveklabnik/rustbook)

- [Documentation](http://steveklabnik.github.io/rustbook/rustbook/)

## Acquiring and building

```
git clone https://github.com/steveklabnik/rustbook.git
cd rustbook
cargo build
```

## Usage

The `rustbook` tool builds a book from a number of separate markdown files. The
contents of the book are determined by a `SUMMARY.md` file like:

```markdown
# Summary

* [Why to use WhizBang](why/README.md)
    * [First reason](why/first.md)
    * [Second reason](why/second.md)
* [How to use WhizBang](how/README.md)
    * [Installing](how/installing.md)
    * [Usage](how/usage.md)
```

The setup is intended to make it easy to browse a book directly on GitHub:

* By convention, each chapter/section with children is placed in its
own subdirectory, with a `README.md` serving as the top level of the
chapter/section.

* Books automatically include an `Introduction` section pointing to the
`README.md` in the root directory.

To build a book, run `rustbook build` in the book's root directory,
which should contain a `SUMMARY.md` and `README.md` as just described.
Currently, the output is always placed in a `_book` subdirectory.
