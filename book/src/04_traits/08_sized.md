# `Sized`

There's more to `&str` than meets the eye, even after having 
investigated deref coercion.  
From our previous [discussion on memory layouts](../03_ticket_v1/10_references_in_memory.md),
it would have been reasonable to expect `&str` to be represented as a single `usize` on
the stack, a pointer. That's not the case though. `&str` stores some **metadata** next 
to the pointer: the length of the slice it points to. Going back to the example from 
[a previous section](05_str_slice.md):

```rust
let mut s = String::with_capacity(5);
s.push_str("Hello");
// Create a string slice reference from the `String`, skipping the first byte.
let slice: &str = &s[1..];
```

In memory, we get:

```text
                    s                              slice
      +---------+--------+----------+      +---------+--------+
Stack | pointer | length | capacity |      | pointer | length |
      |    |    |   5    |    5     |      |    |    |   4    |
      +----|----+--------+----------+      +----|----+--------+
           |        s                           |  
           |                                    |
           v                                    | 
         +---+---+---+---+---+                  |
Heap:    | H | e | l | l | o |                  |
         +---+---+---+---+---+                  |
               ^                                |
               |                                |
               +--------------------------------+
```

What's going on?

## Dynamically sized types

`str` is a **dynamically sized type** (DST).  
A DST is a type whose size is not known at compile time. Whenever you have a 
reference to a DST, like `&str`, it has to include additional
information about the data it points to. It is a **fat pointer**.  
In the case of `&str`, it stores the length of the slice it points to. 
We'll see more examples of DSTs in the rest of the course.

## The `Sized` trait

Rust's `std` library defines a trait called `Sized`.  

```rust
pub trait Sized {
    // This is an empty trait, no methods to implement.
}
```

A type is `Sized` if its size is known at compile time. In other words, it's not a DST.

### Marker traits

`Sized` is your first example of a **marker trait**.  
A marker trait is a trait that doesn't require any methods to be implemented. It doesn't define any behavior.
It only serves to **mark** a type as having certain properties.
The mark is then leveraged by the compiler to enable certain behaviors or optimizations. 

### Auto traits

In particular, `Sized` is also an **auto trait**.  
You don't need to implement it explicitly; the compiler implements it automatically for you
based on the type's definition.

### Examples

All the types we've seen so far are `Sized`: `u32`, `String`, `bool`, etc.

`str`, as we just saw, is not `Sized`.  
`&str` is `Sized` though! We know its size at compile time: two `usize`s, one for the pointer 
and one for the length.

## References

- The exercise for this section is located in `exercises/04_traits/08_sized`
