# Error enums

Your solution to the previous exercise may have felt awkward: matching on strings is not ideal!\
A colleague might rework the error messages returned by `Ticket::new` (e.g. to improve readability) and,
all of a sudden, your calling code would break.

You already know the machinery required to fix this: enums!

## Reacting to errors

When you want to allow the caller to behave differently based on the specific error that occurred, you can
use an enum to represent the different error cases:

```rust
// An error enum to represent the different error cases
// that may occur when parsing a `u32` from a string.
enum U32ParseError {
    NotANumber,
    TooLarge,
    Negative,
}
```

Using an error enum, you're encoding the different error cases in the type system—they become part of the
signature of the fallible function.\
This simplifies error handling for the caller, as they can use a `match` expression to react to the different
error cases:

```rust
match s.parse_u32() {
    Ok(n) => n,
    Err(U32ParseError::Negative) => 0,
    Err(U32ParseError::TooLarge) => u32::MAX,
    Err(U32ParseError::NotANumber) => {
        panic!("Not a number: {}", s);
    }
}
```
