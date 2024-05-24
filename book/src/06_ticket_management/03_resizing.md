# Resizing

We said that `Vec` is a "growable" vector type, but what does that mean?
What happens if you try to insert an element into a `Vec` that's already at maximum capacity?

```rust
let mut numbers = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
numbers.push(3); // Max capacity reached
numbers.push(4); // What happens here?
```

The `Vec` will **resize** itself.\
It will ask the allocator for a new (larger) chunk of heap memory, copy the elements over, and deallocate the old memory.

This operation can be expensive, as it involves a new memory allocation and copying all existing elements.

## `Vec::with_capacity`

If you have a rough idea of how many elements you'll store in a `Vec`, you can use the `Vec::with_capacity`
method to pre-allocate enough memory upfront.\
This can avoid a new allocation when the `Vec` grows, but it may waste memory if you overestimate actual usage.

Evaluate on a case-by-case basis.
