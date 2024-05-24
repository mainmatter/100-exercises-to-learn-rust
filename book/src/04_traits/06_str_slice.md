# String slices

Throughout the previous chapters you've seen quite a few **string literals** being used in the code, 
like `"To-Do"` or `"A ticket description"`.
They were always followed by a call to `.to_string()` or `.into()`. It's time to understand why!

## String literals

You define a string literal by enclosing the raw text in double quotes:

```rust
let s = "Hello, world!";
```

The type of `s` is `&str`, a **reference to a string slice**.  

## Memory layout

`&str` and `String` are different types—they're not interchangeable.  
Let's recall the memory layout of a `String` from our 
[previous exploration](../03_ticket_v1/09_heap.md).
If we run:

```rust
let mut s = String::with_capacity(5);
s.push_str("Hello");
```

we'll get this scenario in memory:

```text
      +---------+--------+----------+
Stack | pointer | length | capacity | 
      |  |      |   5    |    5     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+---+---+
Heap:  | H | e | l | l | o |
       +---+---+---+---+---+
```

If you remember, we've [also examined](../03_ticket_v1/10_references_in_memory.md) 
how a `&String` is laid out in memory:

```text
      --------------------------------------
      |                                    |         
 +----v----+--------+----------+      +----|----+
 | pointer | length | capacity |      | pointer |
 |    |    |   5    |    5     |      |         |
 +----|----+--------+----------+      +---------+
      |        s                          &s 
      |       
      v       
    +---+---+---+---+---+
    | H | e | l | l | o |
    +---+---+---+---+---+
```

`&String` points to the memory location where the `String`'s metadata is stored.  
If we follow the pointer, we get to the heap-allocated data. In particular, we get to the first byte of the string, `H`.

What if we wanted a type that represents a **substring** of `s`? E.g. `ello` in `Hello`?

## String slices

A `&str` is a **view** into a string, a **reference** to a sequence of UTF-8 bytes stored elsewhere.
You can, for example, create a `&str` from a `String` like this:

```rust
let mut s = String::with_capacity(5);
s.push_str("Hello");
// Create a string slice reference from the `String`, skipping the first byte.
let slice: &str = &s[1..];
```

In memory, it'd look like this:

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

`slice` stores two pieces of information on the stack:

- A pointer to the first byte of the slice.
- The length of the slice.

`slice` doesn't own the data, it just points to it: it's a **reference** to the `String`'s heap-allocated data.  
When `slice` is dropped, the heap-allocated data won't be deallocated, because it's still owned by `s`.
That's why `slice` doesn't have a `capacity` field: it doesn't own the data, so it doesn't need to know how much 
space it was allocated for it; it only cares about the data it references.

## `&str` vs `&String`

As a rule of thumb, use `&str` rather than `&String` whenever you need a reference to textual data.  
`&str` is more flexible and generally considered more idiomatic in Rust code.

If a method returns a `&String`, you're promising that there is heap-allocated UTF-8 text somewhere that 
**matches exactly** the one you're returning a reference to.  
If a method returns a `&str`, instead, you have a lot more freedom: you're just saying that *somewhere* there's a 
bunch of text data and that a subset of it matches what you need, therefore you're returning a reference to it.

## References

- The exercise for this section is located in `exercises/04_traits/06_str_slice`
