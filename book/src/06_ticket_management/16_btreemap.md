# Ordering

By moving from a `Vec` to a `HashMap` we have improved the performance of our ticket management system,
and simplified our code in the process.\
It's not all roses, though. When iterating over a `Vec`-backed store, we could be sure that the tickets
would be returned in the order they were added.\
That's not the case with a `HashMap`: you can iterate over the tickets, but the order is random.

We can recover a consistent ordering by switching from a `HashMap` to a `BTreeMap`.

## `BTreeMap`

A `BTreeMap` guarantees that entries are sorted by their keys.\
This is useful when you need to iterate over the entries in a specific order, or if you need to
perform range queries (e.g. "give me all tickets with an id between 10 and 20").

Just like `HashMap`, you won't find trait bounds on the definition of `BTreeMap`.
But you'll find trait bounds on its methods. Let's look at `insert`:

```rust
// `K` and `V` stand for the key and value types, respectively,
// just like in `HashMap`.
impl<K, V> BTreeMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        // implementation
    }
}
```

`Hash` is no longer required. Instead, the key type must implement the `Ord` trait.

## `Ord`

The `Ord` trait is used to compare values.\
While `PartialEq` is used to compare for equality, `Ord` is used to compare for ordering.

It's defined in `std::cmp`:

```rust
pub trait Ord: Eq + PartialOrd {
    fn cmp(&self, other: &Self) -> Ordering;
}
```

The `cmp` method returns an `Ordering` enum, which can be one
of `Less`, `Equal`, or `Greater`.\
`Ord` requires that two other traits are implemented: `Eq` and `PartialOrd`.

## `PartialOrd`

`PartialOrd` is a weaker version of `Ord`, just like `PartialEq` is a weaker version of `Eq`.
You can see why by looking at its definition:

```rust
pub trait PartialOrd: PartialEq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
}
```

`PartialOrd::partial_cmp` returns an `Option`—it is not guaranteed that two values can
be compared.\
For example, `f32` doesn't implement `Ord` because `NaN` values are not comparable,
the same reason why `f32` doesn't implement `Eq`.

## Implementing `Ord` and `PartialOrd`

Both `Ord` and `PartialOrd` can be derived for your types:

```rust
// You need to add `Eq` and `PartialEq` too,
// since `Ord` requires them.
#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct TicketId(u64);
```

If you choose (or need) to implement them manually, be careful:

- `Ord` and `PartialOrd` must be consistent with `Eq` and `PartialEq`.
- `Ord` and `PartialOrd` must be consistent with each other.
