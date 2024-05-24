# Iteration

During the very first exercises, you learned that Rust lets you iterate over collections using `for` loops.
We were looking at ranges at that point (e.g. `0..5`), but the same holds true for collections like arrays and vectors.

```rust
// It works for `Vec`s
let v = vec![1, 2, 3];
for n in v {
    println!("{}", n);
}

// It also works for arrays
let a: [u32; 3] = [1, 2, 3];
for n in a {
    println!("{}", n);
}
```

It's time to understand how this works under the hood.

## `for` desugaring

Every time you write a `for` loop in Rust, the compiler _desugars_ it into the following code:

```rust
let mut iter = IntoIterator::into_iter(v);
loop {
    match iter.next() {
        Some(n) => {
            println!("{}", n);
        }
        None => break,
    }
}
```

`loop` is another looping construct, on top of `for` and `while`.\
A `loop` block will run forever, unless you explicitly `break` out of it.

## `Iterator` trait

The `next` method in the previous code snippet comes from the `Iterator` trait.
The `Iterator` trait is defined in Rust's standard library and provides a shared interface for
types that can produce a sequence of values:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

The `Item` associated type specifies the type of the values produced by the iterator.

`next` returns the next value in the sequence.\
It returns `Some(value)` if there's a value to return, and `None` when there isn't.

Be careful: there is no guarantee that an iterator is exhausted when it returns `None`. That's only
guaranteed if the iterator implements the (more restrictive)
[`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html) trait.

## `IntoIterator` trait

Not all types implement `Iterator`, but many can be converted into a type that does.\
That's where the `IntoIterator` trait comes in:

```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}
```

The `into_iter` method consumes the original value and returns an iterator over its elements.\
A type can only have one implementation of `IntoIterator`: there can be no ambiguity as to what `for` should desugar to.

One detail: every type that implements `Iterator` automatically implements `IntoIterator` as well.
They just return themselves from `into_iter`!

## Bounds checks

Iterating over iterators has a nice side effect: you can't go out of bounds, by design.\
This allows Rust to remove bounds checks from the generated machine code, making iteration faster.

In other words,

```rust
let v = vec![1, 2, 3];
for n in v {
    println!("{}", n);
}
```

is usually faster than

```rust
let v = vec![1, 2, 3];
for i in 0..v.len() {
    println!("{}", v[i]);
}
```

There are exceptions to this rule: the compiler can sometimes prove that you're not going out of bounds even
with manual indexing, thus removing the bounds checks anyway. But in general, prefer iteration to indexing
where possible.
