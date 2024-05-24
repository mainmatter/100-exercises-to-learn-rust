# Implementing traits

When a type is defined in another crate (e.g. `u32`, from Rust's standard library), you
can't directly define new methods for it. If you try:

```rust
impl u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
```

the compiler will complain:

```text
error[E0390]: cannot define inherent `impl` for primitive types
  |
1 | impl u32 {
  | ^^^^^^^^
  |
  = help: consider using an extension trait instead
```

## Extension trait

An **extension trait** is a trait whose primary purpose is to attach new methods
to foreign types, such as `u32`.
That's exactly the pattern you deployed in the previous exercise, by defining
the `IsEven` trait and then implementing it for `i32` and `u32`. You are then
free to call `is_even` on those types as long as `IsEven` is in scope.

```rust
// Bring the trait in scope
use my_library::IsEven;

fn main() {
    // Invoke its method on a type that implements it
    if 4.is_even() {
        // [...]
    }
}
```

## One implementation

There are limitations to the trait implementations you can write.\
The simplest and most straight-forward one: you can't implement the same trait twice,
in a crate, for the same type.

For example:

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        true
    }
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        false
    }
}
```

The compiler will reject it:

```text
error[E0119]: conflicting implementations of trait `IsEven` for type `u32`
   |
5  | impl IsEven for u32 {
   | ------------------- first implementation here
...
11 | impl IsEven for u32 {
   | ^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u32`
```

There can be no ambiguity as to what trait implementation should be used when `IsEven::is_even`
is invoked on a `u32` value, therefore there can only be one.

## Orphan rule

Things get more nuanced when multiple crates are involved.
In particular, at least one of the following must be true:

- The trait is defined in the current crate
- The implementor type is defined in the current crate

This is known as Rust's **orphan rule**. Its goal is to make the method resolution
process unambiguous.

Imagine the following situation:

- Crate `A` defines the `IsEven` trait
- Crate `B` implements `IsEven` for `u32`
- Crate `C` provides a (different) implementation of the `IsEven` trait for `u32`
- Crate `D` depends on both `B` and `C` and calls `1.is_even()`

Which implementation should be used? The one defined in `B`? Or the one defined in `C`?\
There's no good answer, therefore the orphan rule was defined to prevent this scenario.
Thanks to the orphan rule, neither crate `B` nor crate `C` would compile.

## Further reading

- There are some caveats and exceptions to the orphan rule as stated above.
  Check out [the reference](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence)
  if you want to get familiar with its nuances.
