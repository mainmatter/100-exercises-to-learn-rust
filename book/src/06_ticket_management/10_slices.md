# Slices

Let's go back to the memory layout of a `Vec`:

```rust
let mut numbers = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
```

```text
      +---------+--------+----------+
Stack | pointer | length | capacity | 
      |  |      |   2    |    3     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+
Heap:  | 1 | 2 | ? |
       +---+---+---+
```

We already remarked how `String` is just a `Vec<u8>` in disguise.\
The similarity should prompt you to ask: "What's the equivalent of `&str` for `Vec`?"

## `&[T]`

`[T]` is a **slice** of a contiguous sequence of elements of type `T`.\
It's most commonly used in its borrowed form, `&[T]`.

There are various ways to create a slice reference from a `Vec`:

```rust
let numbers = vec![1, 2, 3];
// Via index syntax
let slice: &[i32] = &numbers[..];
// Via a method
let slice: &[i32] = numbers.as_slice();
// Or for a subset of the elements
let slice: &[i32] = &numbers[1..];
```

`Vec` implements the `Deref` trait using `[T]` as the target type, so you can use slice methods on a `Vec` directly
thanks to deref coercion:

```rust
let numbers = vec![1, 2, 3];
// Surprise, surprise: `iter` is not a method on `Vec`!
// It's a method on `&[T]`, but you can call it on a `Vec` 
// thanks to deref coercion.
let sum: i32 = numbers.iter().sum();
```

### Memory layout

A `&[T]` is a **fat pointer**, just like `&str`.\
It consists of a pointer to the first element of the slice and the length of the slice.

If you have a `Vec` with three elements:

```rust
let numbers = vec![1, 2, 3];
```

and then create a slice reference:

```rust
let slice: &[i32] = &numbers[1..];
```

you'll get this memory layout:

```text
                  numbers                          slice
      +---------+--------+----------+      +---------+--------+
Stack | pointer | length | capacity |      | pointer | length |
      |    |    |   3    |    4     |      |    |    |   2    |
      +----|----+--------+----------+      +----|----+--------+
           |                                    |  
           |                                    |
           v                                    | 
         +---+---+---+---+                      |
Heap:    | 1 | 2 | 3 | ? |                      |
         +---+---+---+---+                      |
               ^                                |
               |                                |
               +--------------------------------+
```

### `&Vec<T>` vs `&[T]`

When you need to pass an immutable reference to a `Vec` to a function, prefer `&[T]` over `&Vec<T>`.\
This allows the function to accept any kind of slice, not necessarily one backed by a `Vec`.

For example, you can then pass a subset of the elements in a `Vec`.
But it goes further than that—you could also pass a **slice of an array**:

```rust
let array = [1, 2, 3];
let slice: &[i32] = &array;
```

Array slices and `Vec` slices are the same type: they're fat pointers to a contiguous sequence of elements.
In the case of arrays, the pointer points to the stack rather than the heap, but that doesn't matter
when it comes to using the slice.
