# md_parser
A custom Markdown parser written in Rust

## Overview
A pure Rust implementation of a Markdown parser; with focus on speed and correctness.  We tokenize `.md` files which then forms an abstract syntax tree.

## Building
Install Rust; see the [installation chapter](https://doc.rust-lang.org/book/ch01-01-installation.html) of the Rust Programming Language book.

Compile with:
```
$ git clone https://github.com/caoalbe/md_parser.git
$ cd md_parser
$ cargo build --release
```

## Usage
To parse a file:
```
# This creates README.html
$ ./target/release/md_parser README.md
```

To specify the output file:
```
$ ./target/release/md_parser README.md output.html
```

## Running tests
Run the integration tests with:
```
$ cargo test
```