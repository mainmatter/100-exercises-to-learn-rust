# Modelling A Ticket, pt. 2

The `Ticket` struct we worked on in the previous chapters is a good start,
but it still screams "I'm a beginner Rustacean!".

We'll use this chapter to refine our Rust domain modelling skills.
We'll need to introduce a few more concepts along the way:

- `enum`s, one of Rust's most powerful features for data modeling
- The `Option` type, to model nullable values
- The `Result` type, to model recoverable errors
- The `Debug` and `Display` traits, for printing
- The `Error` trait, to mark error types
- The `TryFrom` and `TryInto` traits, for fallible conversions
- Rust's package system, explaining what's a library, what's a binary, how to use third-party crates
