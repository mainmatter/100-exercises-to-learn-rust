# Arrays

As soon as we start talking about "ticket management" we need to think about a way to store _multiple_ tickets.
In turn, this means we need to think about collections. In particular, homogeneous collections:
we want to store multiple instances of the same type.

What does Rust have to offer in this regard?

## Arrays

A first attempt could be to use an **array**.\
Arrays in Rust are fixed-size collections of elements of the same type.

Here's how you can define an array:

```rust
// Array type syntax: [ <type> ; <number of elements> ]
let numbers: [u32; 3] = [1, 2, 3];
```

This creates an array of 3 integers, initialized with the values `1`, `2`, and `3`.\
The type of the array is `[u32; 3]`, which reads as "an array of `u32`s with a length of 3".

If all array elements are the same, you can use a shorter syntax to initialize it:

```rust
// [ <value> ; <number of elements> ]
let numbers: [u32; 3] = [1; 3];
```

`[1; 3]` creates an array of three elements, all equal to `1`.

### Accessing elements

You can access elements of an array using square brackets:

```rust
let first = numbers[0];
let second = numbers[1];
let third = numbers[2];
```

The index must be of type `usize`.\
Arrays are **zero-indexed**, like everything in Rust. You've seen this before with string slices and field indexing in
tuples/tuple-like variants.

### Out-of-bounds access

If you try to access an element that's out of bounds, Rust will panic:

```rust
let numbers: [u32; 3] = [1, 2, 3];
let fourth = numbers[3]; // This will panic
```

This is enforced at runtime using **bounds checking**. It comes with a small performance overhead, but it's how
Rust prevents buffer overflows.\
In some scenarios the Rust compiler can optimize away bounds checks, especially if iterators are involved—we'll speak
more about this later on.

If you don't want to panic, you can use the `get` method, which returns an `Option<&T>`:

```rust
let numbers: [u32; 3] = [1, 2, 3];
assert_eq!(numbers.get(0), Some(&1));
// You get a `None` if you try to access an out-of-bounds index
// rather than a panic.
assert_eq!(numbers.get(3), None);
```

### Performance

Since the size of an array is known at compile-time, the compiler can allocate the array on the stack.
If you run the following code:

```rust
let numbers: [u32; 3] = [1, 2, 3];
```

You'll get the following memory layout:

```text
        +---+---+---+
Stack:  | 1 | 2 | 3 |
        +---+---+---+
```

In other words, the size of an array is `std::mem::size_of::<T>() * N`, where `T` is the type of the elements and `N` is
the number of elements.\
You can access and replace each element in `O(1)` time.
