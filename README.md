# libcddb-sys

Low-level Rust bindings for the [libcddb](http://libcddb.sourceforge.net/) library. Generated
via [rust-bindgen](https://github.com/rust-lang/rust-bindgen).

## Setup
Install Rust (e.g. via [Rustup](https://www.rust-lang.org/tools/install)), the ordinary
build essentials (e.g. gcc and make), and Clang. On Debian run:
```bash
# apt-get install curl build-essential llvm-dev libclang-dev clang
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

You'll also need libcddb's header files:
```bash
# apt-get install libcddb2-dev
```

## How to build it?
Run:
```bash
$ cargo build
```
