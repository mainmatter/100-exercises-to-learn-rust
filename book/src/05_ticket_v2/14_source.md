# `Error::source`

There's one more thing we need to talk about to complete our coverage of the `Error` trait: the `source` method.

```rust
// Full definition this time!
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
```

The `source` method is a way to access the **error cause**, if any.\
Errors are often chained, meaning that one error is the cause of another: you have a high-level error (e.g.
cannot connect to the database) that is caused by a lower-level error (e.g. can't resolve the database hostname).
The `source` method allows you to "walk" the full chain of errors, often used when capturing error context in logs.

## Implementing `source`

The `Error` trait provides a default implementation that always returns `None` (i.e. no underlying cause). That's why
you didn't have to care about `source` in the previous exercises.\
You can override this default implementation to provide a cause for your error type.

```rust
use std::error::Error;

#[derive(Debug)]
struct DatabaseError {
    source: std::io::Error
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to connect to the database")
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
```

In this example, `DatabaseError` wraps an `std::io::Error` as its source.
We then override the `source` method to return this source when called.

## `&(dyn Error + 'static)`

What's this `&(dyn Error + 'static)` type?\
Let's unpack it:

- `dyn Error` is a **trait object**. It's a way to refer to any type that implements the `Error` trait.
- `'static` is a special **lifetime specifier**.
  `'static` implies that the reference is valid for "as long as we need it", i.e. the entire program execution.

Combined: `&(dyn Error + 'static)` is a reference to a trait object that implements the `Error` trait
and is valid for the entire program execution.

Don't worry too much about either of these concepts for now. We'll cover them in more detail in future chapters.

## Implementing `source` using `thiserror`

`thiserror` provides three ways to automatically implement `source` for your error types:

- A field named `source` will automatically be used as the source of the error.
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          source: std::io::Error
      }
  }
  ```
- A field annotated with the `#[source]` attribute will automatically be used as the source of the error.
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[source]
          inner: std::io::Error
      }
  }
  ```
- A field annotated with the `#[from]` attribute will automatically be used as the source of the error **and**
  `thiserror` will automatically generate a `From` implementation to convert the annotated type into your error type.
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[from]
          inner: std::io::Error
      }
  }
  ```

## The `?` operator

The `?` operator is a shorthand for propagating errors.\
When used in a function that returns a `Result`, it will return early with an error if the `Result` is `Err`.

For example:

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

is equivalent to:

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = match File::open("file.txt") {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        }
    }
    Ok(contents)
}
```

You can use the `?` operator to shorten your error handling code significantly.\
In particular, the `?` operator will automatically convert the error type of the fallible operation into the error type
of the function, if a conversion is possible (i.e. if there is a suitable `From` implementation)
