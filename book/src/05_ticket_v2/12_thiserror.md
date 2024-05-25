# `thiserror`

That was a bit of detour, wasn't it? But a necessary one!\
Let's get back on track now: custom error types and `thiserror`.

## Custom error types

We've seen how to implement the `Error` trait "manually" for a custom error type.\
Imagine that you have to do this for most error types in your codebase. That's a lot of boilerplate, isn't it?

We can remove some of the boilerplate by using [`thiserror`](https://docs.rs/thiserror/latest/thiserror/),
a Rust crate that provides a **procedural macro** to simplify the creation of custom error types.

```rust
#[derive(thiserror::Error, Debug)]
enum TicketNewError {
    #[error("{0}")]
    TitleError(String),
    #[error("{0}")]
    DescriptionError(String),
}
```

## You can write your own macros

All the `derive` macros we've seen so far were provided by the Rust standard library.\
`thiserror::Error` is the first example of a **third-party** `derive` macro.

`derive` macros are a subset of **procedural macros**, a way to generate Rust code at compile time.
We won't get into the details of how to write a procedural macro in this course, but it's important
to know that you can write your own!\
A topic to approach in a more advanced Rust course.

## Custom syntax

Each procedural macro can define its own syntax, which is usually explained in the crate's documentation.
In the case of `thiserror`, we have:

- `#[derive(thiserror::Error)]`: this is the syntax to derive the `Error` trait for a custom error type, helped by `thiserror`.
- `#[error("{0}")]`: this is the syntax to define a `Display` implementation for each variant of the custom error type.
  `{0}` is replaced by the zero-th field of the variant (`String`, in this case) when the error is displayed.
