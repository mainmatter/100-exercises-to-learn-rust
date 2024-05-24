# Lifetimes

Let's try to complete the previous exercise by adding an implementation of `IntoIterator` for `&TicketStore`, for
maximum convenience in `for` loops.

Let's start by filling in the most "obvious" parts of the implementation:

```rust
impl IntoIterator for &TicketStore {
    type Item = &Ticket;
    type IntoIter = // What goes here?

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter()
    }
}
```

What should `type IntoIter` be set to?\
Intuitively, it should be the type returned by `self.tickets.iter()`, i.e. the type returned by `Vec::iter()`.\
If you check the standard library documentation, you'll find that `Vec::iter()` returns an `std::slice::Iter`.
The definition of `Iter` is:

```rust
pub struct Iter<'a, T> { /* fields omitted */ }
```

`'a` is a **lifetime parameter**.

## Lifetime parameters

Lifetimes are **labels** used by the Rust compiler to keep track of how long a reference (either mutable or
immutable) is valid.\
The lifetime of a reference is constrained by the scope of the value it refers to. Rust always makes sure, at compile-time,
that references are not used after the value they refer to has been dropped, to avoid dangling pointers and use-after-free bugs.

This should sound familiar: we've already seen these concepts in action when we discussed ownership and borrowing.
Lifetimes are just a way to **name** how long a specific reference is valid.

Naming becomes important when you have multiple references and you need to clarify how they **relate to each other**.
Let's look at the signature of `Vec::iter()`:

```rust
impl <T> Vec<T> {
    // Slightly simplified
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // [...]
    }
}
```

`Vec::iter()` is generic over a lifetime parameter, named `'a`.\
`'a` is used to **tie together** the lifetime of the `Vec` and the lifetime of the `Iter` returned by `iter()`.
In plain English: the `Iter` returned by `iter()` cannot outlive the `Vec` reference (`&self`) it was created from.

This is important because `Vec::iter`, as we discussed, returns an iterator over **references** to the `Vec`'s elements.
If the `Vec` is dropped, the references returned by the iterator would be invalid. Rust must make sure this doesn't happen,
and lifetimes are the tool it uses to enforce this rule.

## Lifetime elision

Rust has a set of rules, called **lifetime elision rules**, that allow you to omit explicit lifetime annotations in many cases.
For example, `Vec::iter`'s definition looks like this in `std`'s source code:

```rust
impl <T> Vec<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        // [...]
    }
}
```

No explicit lifetime parameter is present in the signature of `Vec::iter()`.
Elision rules imply that the lifetime of the `Iter` returned by `iter()` is tied to the lifetime of the `&self` reference.
You can think of `'_` as a **placeholder** for the lifetime of the `&self` reference.

See the [References](#references) section for a link to the official documentation on lifetime elision.\
In most cases, you can rely on the compiler telling you when you need to add explicit lifetime annotations.

## References

- [std::vec::Vec::iter](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter)
- [std::slice::Iter](https://doc.rust-lang.org/std/slice/struct.Iter.html)
- [Lifetime elision rules](https://doc.rust-lang.org/reference/lifetime-elision.html)
