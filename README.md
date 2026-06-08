# A Rust implementation of a Racket bytecode Virtual Machine

This project is part of a degree's end project, and consists of a VM that aims to be lighter than current official implementation.

To compile it, install Rust and run

`cargo build --release`

A binary with the name "cli" will be created, that is the program entrypoint.

To run it you can either do:

`cli run <FILE>` to run a racket script

or

`cli repl` to enter in a REPL shell