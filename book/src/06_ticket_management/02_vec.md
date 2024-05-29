# Vectors

Arrays' strength is also their weakness: their size must be known upfront, at compile-time.
If you try to create an array with a size that's only known at runtime, you'll get a compilation error:

```rust
let n = 10;
let numbers: [u32; n];
```

```text
error[E0435]: attempt to use a non-constant value in a constant
 --> src/main.rs:3:20
  |
2 | let n = 10;
3 | let numbers: [u32; n];
  |                    ^ non-constant value
```

Arrays wouldn't work for our ticket management system—we don't know how many tickets we'll need to store at compile-time.
This is where `Vec` comes in.

## `Vec`

`Vec` is a growable array type, provided by the standard library.\
You can create an empty array using the `Vec::new` function:

```rust
let mut numbers: Vec<u32> = Vec::new();
```

You would then push elements into the vector using the `push` method:

```rust
numbers.push(1);
numbers.push(2);
numbers.push(3);
```

New values are added to the end of the vector.\
You can also create an initialized vector using the `vec!` macro, if you know the values at creation time:

```rust
let numbers = vec![1, 2, 3];
```

## Accessing elements

The syntax for accessing elements is the same as with arrays:

```rust
let numbers = vec![1, 2, 3];
let first = numbers[0];
let second = numbers[1];
let third = numbers[2];
```

The index must be of type `usize`.\
You can also use the `get` method, which returns an `Option<&T>`:

```rust
let numbers = vec![1, 2, 3];
assert_eq!(numbers.get(0), Some(&1));
// You get a `None` if you try to access an out-of-bounds index
// rather than a panic.
assert_eq!(numbers.get(3), None);
```

Access is bounds-checked, just like element access with arrays. It has O(1) complexity.

## Memory layout

`Vec` is a heap-allocated data structure.\
When you create a `Vec`, it allocates memory on the heap to store the elements.

If you run the following code:

```rust
let mut numbers = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
```

you'll get the following memory layout:

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

`Vec` keeps track of three things:

- The **pointer** to the heap region you reserved.
- The **length** of the vector, i.e. how many elements are in the vector.
- The **capacity** of the vector, i.e. the number of elements that can fit in the space reserved on the heap.

This layout should look familiar: it's exactly the same as `String`!\
That's not a coincidence: `String` is defined as a vector of bytes, `Vec<u8>`, under the hood:

```rust
pub struct String {
    vec: Vec<u8>,
}
```
