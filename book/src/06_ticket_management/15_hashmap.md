# `HashMap`

Our implementation of `Index`/`IndexMut` is not ideal: we need to iterate over the entire
`Vec` to retrieve a ticket by id; the algorithmic complexity is `O(n)`, where
`n` is the number of tickets in the store.

We can do better by using a different data structure for storing tickets: a `HashMap<K, V>`.

```rust
use std::collections::HashMap;

// Type inference lets us omit an explicit type signature (which
// would be `HashMap<String, String>` in this example).
let mut book_reviews = HashMap::new();

book_reviews.insert(
    "Adventures of Huckleberry Finn".to_string(),
    "My favorite book.".to_string(),
);
```

`HashMap` works with key-value pairs. It's generic over both: `K` is the generic
parameter for the key type, while `V` is the one for the value type.

The expected cost of insertions, retrievals and removals is **constant**, `O(1)`.
That sounds perfect for our usecase, doesn't it?

## Key requirements

There are no trait bounds on `HashMap`'s struct definition, but you'll find some
on its methods. Let's look at `insert`, for example:

```rust
// Slightly simplified
impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        // [...]
    }
}
```

The key type must implement the `Eq` and `Hash` traits.\
Let's dig into those two.

## `Hash`

A hashing function (or hasher) maps a potentially infinite set of a values (e.g.
all possible strings) to a bounded range (e.g. a `u64` value).\
There are many different hashing functions around, each with different properties
(speed, collision risk, reversibility, etc.).

A `HashMap`, as the name suggests, uses a hashing function behind the scene.
It hashes your key and then uses that hash to store/retrieve the associated value.
This strategy requires the key type must be hashable, hence the `Hash` trait bound on `K`.

You can find the `Hash` trait in the `std::hash` module:

```rust
pub trait Hash {
    // Required method
    fn hash<H>(&self, state: &mut H)
       where H: Hasher;
}
```

You will rarely implement `Hash` manually. Most of the times you'll derive it:

```rust
#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
}
```

## `Eq`

`HashMap` must be able to compare keys for equality. This is particularly important
when dealing with hash collisions—i.e. when two different keys hash to the same value.

You may wonder: isn't that what the `PartialEq` trait is for? Almost!\
`PartialEq` is not enough for `HashMap` because it doesn't guarantee reflexivity, i.e. `a == a` is always `true`.\
For example, floating point numbers (`f32` and `f64`) implement `PartialEq`,
but they don't satisfy the reflexivity property: `f32::NAN == f32::NAN` is `false`.\
Reflexivity is crucial for `HashMap` to work correctly: without it, you wouldn't be able to retrieve a value
from the map using the same key you used to insert it.

The `Eq` trait extends `PartialEq` with the reflexivity property:

```rust
pub trait Eq: PartialEq {
    // No additional methods
}
```

It's a marker trait: it doesn't add any new methods, it's just a way for you to say to the compiler
that the equality logic implemented in `PartialEq` is reflexive.

You can derive `Eq` automatically when you derive `PartialEq`:

```rust
#[derive(PartialEq, Eq)]
struct Person {
    id: u32,
    name: String,
}
```

## `Eq` and `Hash` are linked

There is an implicit contract between `Eq` and `Hash`: if two keys are equal, their hashes must be equal too.
This is crucial for `HashMap` to work correctly. If you break this contract, you'll get nonsensical results
when using `HashMap`.
