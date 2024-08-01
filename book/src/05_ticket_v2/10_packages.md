# Libraries and binaries

It took a bit of code to implement the `Error` trait for `TicketNewError`, didn't it?\
A manual `Display` implementation, plus an `Error` impl block.

We can remove some of the boilerplate by using [`thiserror`](https://docs.rs/thiserror/latest/thiserror/),
a Rust crate that provides a **procedural macro** to simplify the creation of custom error types.\
But we're getting ahead of ourselves: `thiserror` is a third-party crate, it'd be our first dependency!

Let's take a step back to talk about Rust's packaging system before we dive into dependencies.

## What is a package?

A Rust package is defined by the `[package]` section in a `Cargo.toml` file, also known as its **manifest**.
Within `[package]` you can set the package's metadata, such as its name and version.

Go check the `Cargo.toml` file in the directory of this section's exercise!

## What is a crate?

Inside a package, you can have one or more **crates**, also known as **targets**.\
The two most common crate types are **binary crates** and **library crates**.

### Binaries

A binary is a program that can be compiled to an **executable file**.\
It must include a function named `main`—the program's entry point. `main` is invoked when the program is executed.

### Libraries

Libraries, on the other hand, are not executable on their own. You can't _run_ a library,
but you can _import its code_ from another package that depends on it.\
A library groups together code (i.e. functions, types, etc.) that can be leveraged by other packages as a **dependency**.

All the exercises you've solved so far have been structured as libraries, with a test suite attached to them.

### Conventions

There are some conventions around Rust packages that you need to keep in mind:

- The package's source code is usually located in the `src` directory.
- If there's a `src/lib.rs` file, `cargo` will infer that the package contains a library crate.
- If there's a `src/main.rs` file, `cargo` will infer that the package contains a binary crate.

You can override these defaults by explicitly declaring your targets in the `Cargo.toml` file—see
[`cargo`'s documentation](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#cargo-targets) for more details.

Keep in mind that while a package can contain multiple crates, it can only contain one library crate.
