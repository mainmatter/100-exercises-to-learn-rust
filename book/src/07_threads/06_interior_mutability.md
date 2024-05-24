# Interior mutability

Let's take a moment to reason about the signature of `Sender`'s `send`:

```rust
impl<T> Sender<T> {
    pub fn send(&self, t: T) -> Result<(), SendError<T>> {
        // [...]
    }
}
```

`send` takes `&self` as its argument.\
But it's clearly causing a mutation: it's adding a new message to the channel.
What's even more interesting is that `Sender` is cloneable: we can have multiple instances of `Sender`
trying to modify the channel state **at the same time**, from different threads.

That's the key property we are using to build this client-server architecture. But why does it work?
Doesn't it violate Rust's rules about borrowing? How are we performing mutations via an _immutable_ reference?

## Shared rather than immutable references

When we introduced the borrow-checker, we named the two types of references we can have in Rust:

- immutable references (`&T`)
- mutable references (`&mut T`)

It would have been more accurate to name them:

- shared references (`&T`)
- exclusive references (`&mut T`)

Immutable/mutable is a mental model that works for the vast majority of cases, and it's a great one to get started
with Rust. But it's not the whole story, as you've just seen: `&T` doesn't actually guarantee that the data it
points to is immutable.\
Don't worry, though: Rust is still keeping its promises.
It's just that the terms are a bit more nuanced than they might seem at first.

## `UnsafeCell`

Whenever a type allows you to mutate data through a shared reference, you're dealing with **interior mutability**.

By default, the Rust compiler assumes that shared references are immutable. It **optimises your code** based on that assumption.\
The compiler can reorder operations, cache values, and do all sorts of magic to make your code faster.

You can tell the compiler "No, this shared reference is actually mutable" by wrapping the data in an `UnsafeCell`.\
Every time you see a type that allows interior mutability, you can be certain that `UnsafeCell` is involved,
either directly or indirectly.\
Using `UnsafeCell`, raw pointers and `unsafe` code, you can mutate data through shared references.

Let's be clear, though: `UnsafeCell` isn't a magic wand that allows you to ignore the borrow-checker!\
`unsafe` code is still subject to Rust's rules about borrowing and aliasing.
It's an (advanced) tool that you can leverage to build **safe abstractions** whose safety can't be directly expressed
in Rust's type system. Whenever you use the `unsafe` keyword you're telling the compiler:
"I know what I'm doing, I won't violate your invariants, trust me."

Every time you call an `unsafe` function, there will be documentation explaining its **safety preconditions**:
under what circumstances it's safe to execute its `unsafe` block. You can find the ones for `UnsafeCell`
[in `std`'s documentation](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html).

We won't be using `UnsafeCell` directly in this course, nor will we be writing `unsafe` code.
But it's important to know that it's there, why it exists and how it relates to the types you use
every day in Rust.

## Key examples

Let's go through a couple of important `std` types that leverage interior mutability.\
These are types that you'll encounter somewhat often in Rust code, especially if you peek under the hood of
some the libraries you use.

### Reference counting

`Rc` is a reference-counted pointer.\
It wraps around a value and keeps track of how many references to the value exist.
When the last reference is dropped, the value is deallocated.\
The value wrapped in an `Rc` is immutable: you can only get shared references to it.

```rust
use std::rc::Rc;

let a: Rc<String> = Rc::new("My string".to_string());
// Only one reference to the string data exists.
assert_eq!(Rc::strong_count(&a), 1);

// When we call `clone`, the string data is not copied!
// Instead, the reference count for `Rc` is incremented.
let b = Rc::clone(&a);
assert_eq!(Rc::strong_count(&a), 2);
assert_eq!(Rc::strong_count(&b), 2);
// ^ Both `a` and `b` point to the same string data
//   and share the same reference counter.
```

`Rc` uses `UnsafeCell` internally to allow shared references to increment and decrement the reference count.

### `RefCell`

`RefCell` is one of the most common examples of interior mutability in Rust.
It allows you to mutate the value wrapped in a `RefCell` even if you only have an
immutable reference to the `RefCell` itself.

This is done via **runtime borrow checking**.
The `RefCell` keeps track of the number (and type) of references to the value it contains at runtime.
If you try to borrow the value mutably while it's already borrowed immutably,
the program will panic, ensuring that Rust's borrowing rules are always enforced.

```rust
use std::cell::RefCell;

let x = RefCell::new(42);

let y = x.borrow(); // Immutable borrow
let z = x.borrow_mut(); // Panics! There is an active immutable borrow.
```
