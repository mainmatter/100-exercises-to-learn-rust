# Mutable slices

Every time we've talked about slice types (like `str` and `[T]`), we've used their immutable borrow form (`&str` and `&[T]`).\
But slices can also be mutable!

Here's how you create a mutable slice:

```rust
let mut numbers = vec![1, 2, 3];
let slice: &mut [i32] = &mut numbers;
```

You can then modify the elements in the slice:

```rust
slice[0] = 42;
```

This will change the first element of the `Vec` to `42`.

## Limitations

When working with immutable borrows, the recommendation was clear: prefer slice references over references to
the owned type (e.g. `&[T]` over `&Vec<T>`).\
That's **not** the case with mutable borrows.

Consider this scenario:

```rust
let mut numbers = Vec::with_capacity(2);
let mut slice: &mut [i32] = &mut numbers;
slice.push(1);
```

It won't compile!\
`push` is a method on `Vec`, not on slices. This is the manifestation of a more general principle: Rust won't
allow you to add or remove elements from a slice. You will only be able to modify/replace the elements that are
already there.

In this regard, a `&mut Vec` or a `&mut String` are strictly more powerful than a `&mut [T]` or a `&mut str`.\
Choose the type that best fits based on the operations you need to perform.
