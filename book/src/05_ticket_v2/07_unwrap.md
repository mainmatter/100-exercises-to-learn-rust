# Unwrapping

`Ticket::new` now returns a `Result` instead of panicking on invalid inputs.\
What does this mean for the caller?

## Failures can't be (implicitly) ignored

Unlike exceptions, Rust's `Result` forces you to **handle errors at the call site**.\
If you call a function that returns a `Result`, Rust won't allow you to implicitly ignore the error case.

```rust
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}

// This won't compile: we're not handling the error case.
// We must either use `match` or one of the combinators provided by 
// `Result` to "unwrap" the success value or handle the error.
let number = parse_int("42") + 2;
```

## You got a `Result`. Now what?

When you call a function that returns a `Result`, you have two key options:

- Panic if the operation failed.
  This is done using either the `unwrap` or `expect` methods.
  ```rust
  // Panics if `parse_int` returns an `Err`.
  let number = parse_int("42").unwrap();
  // `expect` lets you specify a custom panic message.
  let number = parse_int("42").expect("Failed to parse integer");
  ```
- Destructure the `Result` using a `match` expression to deal with the error case explicitly.
  ```rust
  match parse_int("42") {
      Ok(number) => println!("Parsed number: {}", number),
      Err(err) => eprintln!("Error: {}", err),
  }
  ```
