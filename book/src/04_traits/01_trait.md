# Traits

Let's look again at our `Ticket` type:

```rust
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

All our tests, so far, have been making assertions using `Ticket`'s fields.

```rust
assert_eq!(ticket.title(), "A new title");
```

What if we wanted to compare two `Ticket` instances directly?

```rust
let ticket1 = Ticket::new(/* ... */);
let ticket2 = Ticket::new(/* ... */);
ticket1 == ticket2
```

The compiler will stop us:

```text
error[E0369]: binary operation `==` cannot be applied to type `Ticket`
  --> src/main.rs:18:13
   |
18 |     ticket1 == ticket2
   |     ------- ^^ ------- Ticket
   |     |
   |     Ticket
   |
note: an implementation of `PartialEq` might be missing for `Ticket`
```

`Ticket` is a new type. Out of the box, there is **no behavior attached to it**.\
Rust doesn't magically infer how to compare two `Ticket` instances just because they contain `String`s.

The Rust compiler is nudging us in the right direction though: it's suggesting that we might be missing an implementation
of `PartialEq`. `PartialEq` is a **trait**!

## What are traits?

Traits are Rust's way of defining **interfaces**.\
A trait defines a set of methods that a type must implement to satisfy the trait's contract.

### Defining a trait

The syntax for a trait definition goes like this:

```rust
trait <TraitName> {
    fn <method_name>(<parameters>) -> <return_type>;
}
```

We might, for example, define a trait named `MaybeZero` that requires its implementors to define an `is_zero` method:

```rust
trait MaybeZero {
    fn is_zero(self) -> bool;
}
```

### Implementing a trait

To implement a trait for a type we use the `impl` keyword, just like we do for regular[^inherent] methods,
but the syntax is a bit different:

```rust
impl <TraitName> for <TypeName> {
    fn <method_name>(<parameters>) -> <return_type> {
        // Method body
    }
}
```

For example, to implement the `MaybeZero` trait for a custom number type, `WrappingU32`:

```rust
pub struct WrappingU32 {
    inner: u32,
}

impl MaybeZero for WrappingU32 {
    fn is_zero(self) -> bool {
        self.inner == 0
    }
}
```

### Invoking a trait method

To invoke a trait method, we use the `.` operator, just like we do with regular methods:

```rust
let x = WrappingU32 { inner: 5 };
assert!(!x.is_zero());
```

To invoke a trait method, two things must be true:

- The type must implement the trait.
- The trait must be in scope.

To satisfy the latter, you may have to add a `use` statement for the trait:

```rust
use crate::MaybeZero;
```

This is not necessary if:

- The trait is defined in the same module where the invocation occurs.
- The trait is defined in the standard library's **prelude**.
  The prelude is a set of traits and types that are automatically imported into every Rust program.
  It's as if `use std::prelude::*;` was added at the beginning of every Rust module.

You can find the list of traits and types in the prelude in the
[Rust documentation](https://doc.rust-lang.org/std/prelude/index.html).

[^inherent]: A method defined directly on a type, without using a trait, is also known as an **inherent method**.
