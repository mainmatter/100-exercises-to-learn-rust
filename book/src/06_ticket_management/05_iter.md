# `.iter()`

`IntoIterator` **consumes** `self` to create an iterator.

This has its benefits: you get **owned** values from the iterator.
For example: if you call `.into_iter()` on a `Vec<Ticket>` you'll get an iterator that returns `Ticket` values.

That's also its downside: you can no longer use the original collection after calling `.into_iter()` on it.
Quite often you want to iterate over a collection without consuming it, looking at **references** to the values instead.
In the case of `Vec<Ticket>`, you'd want to iterate over `&Ticket` values.

Most collections expose a method called `.iter()` that returns an iterator over references to the collection's elements.
For example:

```rust
let numbers: Vec<u32> = vec![1, 2];
// `n` has type `&u32` here
for n in numbers.iter() {
    // [...]
}
```

This pattern can be simplified by implementing `IntoIterator` for a **reference to the collection**.
In our example above, that would be `&Vec<Ticket>`.\
The standard library does this, that's why the following code works:

```rust
let numbers: Vec<u32> = vec![1, 2];
// `n` has type `&u32` here
// We didn't have to call `.iter()` explicitly
// It was enough to use `&numbers` in the `for` loop
for n in &numbers {
    // [...]
}
```

It's idiomatic to provide both options:

- An implementation of `IntoIterator` for a reference to the collection.
- An `.iter()` method that returns an iterator over references to the collection's elements.

The former is convenient in `for` loops, the latter is more explicit and can be used in other contexts.
