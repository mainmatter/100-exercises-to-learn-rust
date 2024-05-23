# References

What about references, like `&String` or `&mut String`? How are they represented in memory?

Most references[^fat] in Rust are represented, in memory, as a pointer to a memory location.  
It follows that their size is the same as the size of a pointer, a `usize`.

You can verify this using `std::mem::size_of`:

```rust
assert_eq!(std::mem::size_of::<&String>(), 8);
assert_eq!(std::mem::size_of::<&mut String>(), 8);
```

A `&String`, in particular, is a pointer to the memory location where the `String`'s metadata is stored.  
If you run this snippet:

```rust
let s = String::from("Hey");
let r = &s;
```

you'll get something like this in memory:

```
           --------------------------------------
           |                                    |
      +----v----+--------+----------+      +----|----+
Stack | pointer | length | capacity |      | pointer |
      |  |      |   3    |    5     |      |         |
      +--|  ----+--------+----------+      +---------+
         |          s                           r
         |
         v
       +---+---+---+---+---+
Heap   | H | e | y | ? | ? |
       +---+---+---+---+---+
```

It's a pointer to a pointer to the heap-allocated data, if you will.
The same goes for `&mut String`. 

## Not all pointers point to the heap

The example above should clarify one thing: not all pointers point to the heap.  
They just point to a memory location, which _may_ be on the heap, but doesn't have to be.

## References

- The exercise for this section is located in `exercises/03_ticket_v1/10_references_in_memory`

[^fat]: [Later in the course](../04_traits/05_str_slice) we'll talk about **fat pointers**,
i.e. pointers with additional metadata. As the name implies, they are larger than
the pointers we discussed in this chapter, also known as **thin pointers**.
