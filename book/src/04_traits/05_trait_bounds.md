# Trait bounds

We've seen two use cases for traits so far:

- Unlocking "built-in" behaviour (e.g. operator overloading)
- Adding new behaviour to existing types (i.e. extension traits)

There's a third use case: **generic programming**.

## The problem

All our functions and methods, so far, have been working with **concrete types**.\
Code that operates on concrete types is usually straightforward to write and understand. But it's also
limited in its reusability.\
Let's imagine, for example, that we want to write a function that returns `true` if an integer is even.
Working with concrete types, we'd have to write a separate function for each integer type we want to
support:

```rust
fn is_even_i32(n: i32) -> bool {
    n % 2 == 0
}

fn is_even_i64(n: i64) -> bool {
    n % 2 == 0
}

// Etc.
```

Alternatively, we could write a single extension trait and then different implementations for each integer type:

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl IsEven for i64 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

// Etc.
```

The duplication remains.

## Generic programming

We can do better using **generics**.\
Generics allow us to write code that works with a **type parameter** instead of a concrete type:

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
{
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

`print_if_even` is a **generic function**.\
It isn't tied to a specific input type. Instead, it works with any type `T` that:

- Implements the `IsEven` trait.
- Implements the `Debug` trait.

This contract is expressed with a **trait bound**: `T: IsEven + Debug`.\
The `+` symbol is used to require that `T` implements multiple traits. `T: IsEven + Debug` is equivalent to
"where `T` implements `IsEven` **and** `Debug`".

## Trait bounds

What purpose do trait bounds serve in `print_if_even`?\
To find out, let's try to remove them:

```rust
fn print_if_even<T>(n: T) {
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

This code won't compile:

```text
error[E0599]: no method named `is_even` found for type parameter `T` in the current scope
 --> src/lib.rs:2:10
  |
1 | fn print_if_even<T>(n: T) {
  |                  - method `is_even` not found for this type parameter
2 |     if n.is_even() {
  |          ^^^^^^^ method not found in `T`

error[E0277]: `T` doesn't implement `Debug`
 --> src/lib.rs:3:19
  |
3 |         println!("{n:?} is even");
  |                   ^^^^^ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`
  |
help: consider restricting type parameter `T`
  |
1 | fn print_if_even<T: std::fmt::Debug>(n: T) {
  |                   +++++++++++++++++
```

Without trait bounds, the compiler doesn't know what `T` **can do**.\
It doesn't know that `T` has an `is_even` method, and it doesn't know how to format `T` for printing.
From the compiler point of view, a bare `T` has no behaviour at all.\
Trait bounds restrict the set of types that can be used by ensuring that the behaviour required by the function
body is present.

## Syntax: inlining trait bounds

All the examples above used a **`where` clause** to specify trait bounds:

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
//  ^^^^^^^^^^^^^^^^^
//  This is a `where` clause
{
    // [...]
}
```

If the trait bounds are simple, you can **inline** them directly next to the type parameter:

```rust
fn print_if_even<T: IsEven + Debug>(n: T) {
    //           ^^^^^^^^^^^^^^^^^
    //           This is an inline trait bound
    // [...]
}
```

## Syntax: meaningful names

In the examples above, we used `T` as the type parameter name. This is a common convention when a function has
only one type parameter.\
Nothing stops you from using a more meaningful name, though:

```rust
fn print_if_even<Number: IsEven + Debug>(n: Number) {
    // [...]
}
```

It is actually **desirable** to use meaningful names when there are multiple type parameters at play or when the name
`T` doesn't convey enough information about the type's role in the function.
Maximize clarity and readability when naming type parameters, just as you would with variables or function parameters.
Follow Rust's conventions, though: use [upper camel case for type parameter names](https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-rfc-430-c-case).

## The function signature is king

You may wonder why we need trait bounds at all. Can't the compiler infer the required traits from the function's body?\
It could, but it won't.\
The rationale is the same as for [explicit type annotations on function parameters](../02_basic_calculator/02_variables.md#function-arguments-are-variables):
each function signature is a contract between the caller and the callee, and the terms must be explicitly stated.
This allows for better error messages, better documentation, less unintentional breakages across versions,
and faster compilation times.
