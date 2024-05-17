# Panics

Let's go back to the `speed` function you wrote for the ["Variables" section](../02_variables/README.md).
It probably looked something like this:

```rust
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    let distance = end - start;
    distance / time_elapsed
}
```

If you have a keen eye, you might have spotted one issue[^one]: what happens if `time_elapsed` is zero?

You can try it
out [on the Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=36e5ddbe3b3f741dfa9f74c956622bac)!  
The program will exit with the following error message:

```text
thread 'main' panicked at src/main.rs:3:5:
attempt to divide by zero
```

This is known as a **panic**.  
A panic is Rust's way to signal that something went so wrong that
the program can't continue executing, it's an **unrecoverable error**[^catching]. Division by zero classifies as such an
error.

## The panic! macro

You can intentionally trigger a panic by calling the `panic!` macro[^macro]:

```rust
fn main() {
    panic!("This is a panic!");
    // The line below will never be executed
    let x = 1 + 2;
}
```

There are other mechanisms to work with recoverable errors in Rust, which [we'll cover later](../05_ticket_v2/06_fallibility).
For the time being we'll stick with panics as a brutal but simple stopgap solution.

## References

- The exercise for this section is located in `exercises/02_basic_calculator/04_panics`

## Further reading

- [The panic! macro documentation](https://doc.rust-lang.org/std/macro.panic.html)

[^one]: There's another issue with `speed` that we'll address soon enough. Can you spot it?

[^catching]: You can try to catch a panic, but it should be a last resort attempt reserved for very specific
circumstances.

[^macro]: If it's followed by a `!`, it's a macro invocation. Think of macros as spicy functions for now. We'll
cover them in more detail later in the course.
