# Indexing

`TicketStore::get` returns an `Option<&Ticket>` for a given `TicketId`.\
We've seen before how to access elements of arrays and vectors using Rust's
indexing syntax:

```rust
let v = vec![0, 1, 2];
assert_eq!(v[0], 0);
```

How can we provide the same experience for `TicketStore`?\
You guessed right: we need to implement a trait, `Index`!

## `Index`

The `Index` trait is defined in Rust's standard library:

```rust
// Slightly simplified
pub trait Index<Idx>
{
    type Output;

    // Required method
    fn index(&self, index: Idx) -> &Self::Output;
}
```

It has:

- One generic parameter, `Idx`, to represent the index type
- One associated type, `Output`, to represent the type we retrieved using the index

Notice how the `index` method doesn't return an `Option`. The assumption is that
`index` will panic if you try to access an element that's not there, as it happens
for array and vec indexing.
