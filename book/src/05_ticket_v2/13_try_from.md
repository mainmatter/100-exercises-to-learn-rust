# `TryFrom` and `TryInto`

In the previous chapter we looked at the [`From` and `Into` traits](../04_traits/09_from.md),
Rust's idiomatic interfaces for **infallible** type conversions.\
But what if the conversion is not guaranteed to succeed?

We now know enough about errors to discuss the **fallible** counterparts of `From` and `Into`:
`TryFrom` and `TryInto`.

## `TryFrom` and `TryInto`

Both `TryFrom` and `TryInto` are defined in the `std::convert` module, just like `From` and `Into`.

```rust
pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```

The main difference between `From`/`Into` and `TryFrom`/`TryInto` is that the latter return a `Result` type.\
This allows the conversion to fail, returning an error instead of panicking.

## `Self::Error`

Both `TryFrom` and `TryInto` have an associated `Error` type.
This allows each implementation to specify its own error type, ideally the most appropriate for the conversion
being attempted.

`Self::Error` is a way to refer to the `Error` associated type defined in the trait itself.

## Duality

Just like `From` and `Into`, `TryFrom` and `TryInto` are dual traits.\
If you implement `TryFrom` for a type, you get `TryInto` for free.
