# Fallibility

Let's revisit the `Ticket::new` function from the previous exercise:

```rust
impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: Status
    ) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }

        Ticket {
            title,
            description,
            status,
        }
    }
}
```

As soon as one of the checks fails, the function panics.
This is not ideal, as it doesn't give the caller a chance to **handle the error**.

It's time to introduce the `Result` type, Rust's primary mechanism for error handling.

## The `Result` type

The `Result` type is an enum defined in the standard library:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

It has two variants:

- `Ok(T)`: represents a successful operation. It holds `T`, the output of the operation.
- `Err(E)`: represents a failed operation. It holds `E`, the error that occurred.

Both `Ok` and `Err` are generic, allowing you to specify your own types for the success and error cases.

## No exceptions

Recoverable errors in Rust are **represented as values**.\
They're just an instance of a type, being passed around and manipulated like any other value.
This is a significant difference from other languages, such as Python or C#, where **exceptions** are used to signal errors.

Exceptions create a separate control flow path that can be hard to reason about.\
You don't know, just by looking at a function's signature, if it can throw an exception or not.
You don't know, just by looking at a function's signature, **which** exception types it can throw.\
You must either read the function's documentation or look at its implementation to find out.

Exception handling logic has very poor locality: the code that throws the exception is far removed from the code
that catches it, and there's no direct link between the two.

## Fallibility is encoded in the type system

Rust, with `Result`, forces you to **encode fallibility in the function's signature**.\
If a function can fail (and you want the caller to have a shot at handling the error), it must return a `Result`.

```rust
// Just by looking at the signature, you know that this function 
// can fail. You can also inspect `ParseIntError` to see what 
// kind of failures to expect.
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}
```

That's the big advantage of `Result`: it makes fallibility explicit.

Keep in mind, though, that panics exist. They aren't tracked by the type system, just like exceptions in other languages.
But they're meant for **unrecoverable errors** and should be used sparingly.
