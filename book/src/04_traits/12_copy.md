# Copying values, pt. 2

Let's consider the same example as before, but with a slight twist: using `u32` rather than `String` as a type.

```rust
fn consumer(s: u32) { /* */ }

fn example() {
     let s: u32 = 5;
     consumer(s);
     let t = s + 1;
}
```

It'll compile without errors! What's going on here? What's the difference between `String` and `u32` 
that makes the latter work without `.clone()`?

## `Copy`

`Copy` is another trait defined in Rust's standard library:

```rust
pub trait Copy: Clone { }
```

It is a marker trait, just like `Sized`.

If a type implements `Copy`, there's no need to call `.clone()` to create a new instance of the type:
Rust does it **implicitly** for you.  
`u32` is an example of a type that implements `Copy`, which is why the example above compiles without errors:
when `consumer(s)` is called, Rust creates a new `u32` instance by performing a **bitwise copy** of `s`, 
and then passes that new instance to `consumer`. It all happens behind the scenes, without you having to do anything.

## What can be `Copy`?

`Copy` is not equivalent to "automatic cloning", although it implies it.  
Types must meet a few requirements in order to be allowed to implement `Copy`.

First of all, it must implement `Clone`, since `Copy` is a subtrait of `Clone`.
This makes sense: if Rust can create a new instance of a type _implicitly_, it should 
also be able to create a new instance _explicitly_ by calling `.clone()`.

That's not all, though. A few more conditions must be met:

1. The type doesn't manage any _additional_ resources (e.g. heap memory, file handles, etc.) beyond the `std::mem::size_of`
   bytes that it occupies in memory. 
2. The type is not a mutable reference (`&mut T`).

If both conditions are met, then Rust can safely create a new instance of the type by performing a **bitwise copy** 
of the original instance—this is often referred to as a `memcpy` operation, after the C standard library function
that performs the bitwise copy.

### Case study 1: `String`

`String` is a type that doesn't implement `Copy`.  
Why? Because it manages an additional resource: the heap-allocated memory buffer that stores the string's data.

Let's imagine that Rust allowed `String` to implement `Copy`.  
Then, when a new `String` instance is created by performing a bitwise copy of the original instance, both the original
and the new instance would point to the same memory buffer: 

```text
               s                                 copied_s
 +---------+--------+----------+      +---------+--------+----------+
 | pointer | length | capacity |      | pointer | length | capacity |
 |  |      |   5    |    5     |      |  |      |   5    |    5     |
 +--|------+--------+----------+      +--|------+--------+----------+
    |                                    |
    |                                    |
    v                                    |
  +---+---+---+---+---+                  |
  | H | e | l | l | o |                  |
  +---+---+---+---+---+                  |
    ^                                    |
    |                                    |
    +------------------------------------+
```

This is bad!
Both `String` instances would try to free the memory buffer when they go out of scope, 
leading to a double-free error.
You could also create two distinct `&mut String` references that point to the same memory buffer,
violating Rust's borrowing rules.

### Case study 2: `u32`

`u32` implements `Copy`. All integer types do, in fact.  
An integer is "just" the bytes that represent the number in memory. There's nothing more!
If you copy those bytes, you get another perfectly valid integer instance.
Nothing bad can happen, so Rust allows it.

### Case study 3: `&mut u32`

When we introduced ownership and mutable borrows, we stated one rule quite clearly: there
can only ever be *one* mutable borrow of a value at any given time.  
That's why `&mut u32` doesn't implement `Copy`, even though `u32` does.

If `&mut u32` implemented `Copy`, you could create multiple mutable references to 
the same value and modify it in multiple places at the same time.
That'd be a violation of Rust's borrowing rules! 
It follows that `&mut T` never implements `Copy`, no matter what `T` is.

## Implementing `Copy`

In most cases, you don't need to manually implement `Copy`.
You can just derive it, like this:

```rust
#[derive(Copy, Clone)]
struct MyStruct {
    field: u32,
}
```

## References

- The exercise for this section is located in `exercises/04_traits/12_copy`
