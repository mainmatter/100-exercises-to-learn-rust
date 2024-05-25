# Loops, part 1: `while`

Your implementation of `factorial` has been forced to use recursion.\
This may feel natural to you, especially if you're coming from a functional programming background.
Or it may feel strange, if you're used to more imperative languages like C or Python.

Let's see how you can implement the same functionality using a **loop** instead.

## The `while` loop

A `while` loop is a way to execute a block of code as long as a **condition** is true.\
Here's the general syntax:

```rust
while <condition> {
    // code to execute
}
```

For example, we might want to sum the numbers from 1 to 5:

```rust
let sum = 0;
let i = 1;
// "while i is less than or equal to 5"
while i <= 5 {
    // `+=` is a shorthand for `sum = sum + i`
    sum += i;
    i += 1;
}
```

This will keep adding 1 to `i` and `i` to `sum` until `i` is no longer less than or equal to 5.

## The `mut` keyword

The example above won't compile as is. You'll get an error like:

```text
error[E0384]: cannot assign twice to immutable variable `sum`
 --> src/main.rs:7:9
  |
2 |     let sum = 0;
  |         ---
  |         |
  |         first assignment to `sum`
  |         help: consider making this binding mutable: `mut sum`
...
7 |         sum += i;
  |         ^^^^^^^^ cannot assign twice to immutable variable

error[E0384]: cannot assign twice to immutable variable `i`
 --> src/main.rs:8:9
  |
3 |     let i = 1;
  |         -
  |         |
  |         first assignment to `i`
  |         help: consider making this binding mutable: `mut i`
...
8 |         i += 1;
  |         ^^^^^^ cannot assign twice to immutable variable
```

This is because variables in Rust are **immutable** by default.\
You can't change their value once it has been assigned.

If you want to allow modifications, you have to declare the variable as **mutable** using the `mut` keyword:

```rust
// `sum` and `i` are mutable now!
let mut sum = 0;
let mut i = 1;

while i <= 5 {
    sum += i;
    i += 1;
}
```

This will compile and run without errors.

## Further reading

- [`while` loop documentation](https://doc.rust-lang.org/std/keyword.while.html)
