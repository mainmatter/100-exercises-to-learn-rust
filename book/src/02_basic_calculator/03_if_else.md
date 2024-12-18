# Control flow, part 1

All our programs so far have been pretty straightforward.\
A sequence of instructions is executed from top to bottom, and that's it.

It's time to introduce some **branching**.

## `if` clauses

The `if` keyword is used to execute a block of code only if a condition is true.

Here's a simple example:

```rust
let number = 3;
if number < 5 {
    println!("`number` is smaller than 5");
}
```

This program will print `number is smaller than 5` because the condition `number < 5` is true.

### `else` clauses

Like most programming languages, Rust supports an optional `else` branch to execute a block of code when the condition in an
`if` expression is false.\
For example:

```rust
let number = 3;

if number < 5 {
    println!("`number` is smaller than 5");
} else {
    println!("`number` is greater than or equal to 5");
}
```

### `else if` clauses

Your code drifts more and more to the right when you have multiple `if` expressions, one nested inside the other.

```rust
let number = 3;

if number < 5 {
    println!("`number` is smaller than 5");
} else {
    if number >= 3 {
        println!("`number` is greater than or equal to 3, but smaller than 5");
    } else {
        println!("`number` is smaller than 3");
    }
}
```

You can use the `else if` keyword to combine multiple `if` expressions into a single one:

```rust
let number = 3;

if number < 5 {
    println!("`number` is smaller than 5");
} else if number >= 3 {
    println!("`number` is greater than or equal to 3, but smaller than 5");
} else {
    println!("`number` is smaller than 3");
}
```

## Booleans

The condition in an `if` expression must be of type `bool`, a **boolean**.\
Booleans, just like integers, are a primitive type in Rust.

A boolean can have one of two values: `true` or `false`.

### No truthy or falsy values

If the condition in an `if` expression is not a boolean, you'll get a compilation error.

For example, the following code will not compile:

```rust
let number = 3;
if number {
    println!("`number` is not zero");
}
```

You'll get the following compilation error:

```text
error[E0308]: mismatched types
 --> src/main.rs:3:8
  |
3 |     if number {
  |        ^^^^^^ expected `bool`, found integer
```

This follows from Rust's philosophy around type coercion: there's no automatic conversion from non-boolean types to booleans.
Rust doesn't have the concept of **truthy** or **falsy** values, like JavaScript or Python.\
You have to be explicit about the condition you want to check.

### Comparison operators

It's quite common to use comparison operators to build conditions for `if` expressions.\
Here are the comparison operators available in Rust when working with integers:

- `==`: equal to
- `!=`: not equal to
- `<`: less than
- `>`: greater than
- `<=`: less than or equal to
- `>=`: greater than or equal to

## `if/else` is an expression

In Rust, `if` expressions are **expressions**, not statements: they return a value.\
That value can be assigned to a variable or used in other expressions. For example:

```rust
let number = 3;
let message = if number < 5 {
    "smaller than 5"
} else {
    "greater than or equal to 5"
};
```

In the example above, each branch of the `if` evaluates to a string literal,
which is then assigned to the `message` variable.\
The only requirement is that both `if` branches return the same type.
