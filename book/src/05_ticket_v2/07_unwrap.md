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

When you call a function that returns a `Result`, you have three key options:

- Destructure the `Result` using a `match` expression to deal with the error case explicitly.
  ```rust
  match parse_int("42") {
      Ok(number) => println!("Parsed number: {}", number),
      Err(err) => eprintln!("Error: {}", err),
  }
  ```
- Propagate the error. If you're in a function which also returns a `Result`, you can use the `?` operator to early-return an error if it occurred:
  ```rust
  let number = parse_int("42")?;
  println!("Parsed number: {}", number);
  ```

  The `?` operator works the same as manually destructuring the result and early returning:
  ```rust
  let number = match parse_int("42") {
      Ok(number) => number,
      Err(err) => return Err(err),
  };
  println!("Parsed number: {}", number);
  ```
- Panic if the operation failed.\
  This is done using either the `unwrap` or `expect` methods.\
  This this should generally only be done in your top-level `main` function - most functions should propagate errors rather than panic.
  ```rust
  // Panics if `parse_int` returns an `Err`.
  let number = parse_int("42").unwrap();
  // `expect` lets you specify a custom panic message.
  let number = parse_int("42").expect("Failed to parse integer");
  ```
