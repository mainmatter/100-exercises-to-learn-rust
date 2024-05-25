# Error trait

## Error reporting

In the previous exercise you had to destructure the `TitleError` variant to extract the error message and
pass it to the `panic!` macro.\
This is a (rudimentary) example of **error reporting**: transforming an error type into a representation that can be
shown to a user, a service operator, or a developer.

It's not practical for each Rust developer to come up with their own error reporting strategy: it'd a waste of time
and it wouldn't compose well across projects.
That's why Rust provides the `std::error::Error` trait.

## The `Error` trait

There are no constraints on the type of the `Err` variant in a `Result`, but it's a good practice to use a type
that implements the `Error` trait.
`Error` is the cornerstone of Rust's error handling story:

```rust
// Slightly simplified definition of the `Error` trait
pub trait Error: Debug + Display {}
```

You might recall the `:` syntax from [the `Sized` trait](../04_traits/08_sized.md)—it's used to specify **supertraits**.
For `Error`, there are two supertraits: `Debug` and `Display`. If a type wants to implement `Error`, it must also
implement `Debug` and `Display`.

## `Display` and `Debug`

We've already encountered the `Debug` trait in [a previous exercise](../04_traits/04_derive.md)—it's the trait used by
`assert_eq!` to display the values of the variables it's comparing when the assertion fails.

From a "mechanical" perspective, `Display` and `Debug` are identical—they encode how a type should be converted
into a string-like representation:

```rust
// `Debug`
pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}

// `Display`
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

The difference is in their _purpose_: `Display` returns a representation that's meant for "end-users",
while `Debug` provides a low-level representation that's more suitable to developers and service operators.\
That's why `Debug` can be automatically implemented using the `#[derive(Debug)]` attribute, while `Display`
**requires** a manual implementation.
