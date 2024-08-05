# Memory layout

We've looked at ownership and references from an operational point of view—what you can and can't do with them.
Now it's a good time to take a look under the hood: let's talk about **memory**.

## Stack and heap

When discussing memory, you'll often hear people talk about the **stack** and the **heap**.\
These are two different memory regions used by programs to store data.

Let's start with the stack.

## Stack

The **stack** is a **LIFO** (Last In, First Out) data structure.\
When you call a function, a new **stack frame** is added on top of the stack. That stack frame stores
the function's arguments, local variables and a few "bookkeeping" values.\
When the function returns, the stack frame is popped off the stack[^stack-overflow].

```text
+-----------------+
| frame for func1 |
+-----------------+
        |
        | func2 is 
        | called
        v
+-----------------+
| frame for func2 |
+-----------------+
| frame for func1 |
+-----------------+
        |
        | func2  
        | returns
        v
+-----------------+
| frame for func1 |
+-----------------+
```

From an operational point of view, stack allocation/de-allocation is **very fast**.\
We are always pushing and popping data from the top of the stack, so we don't need to search for free memory.
We also don't have to worry about fragmentation: the stack is a single contiguous block of memory.

### Rust

Rust will often allocate data on the stack.\
You have a `u32` input argument in a function? Those 32 bits will be on the stack.\
You define a local variable of type `i64`? Those 64 bits will be on the stack.\
It all works quite nicely because the size of those integers is known at compile time, therefore
the compiled program knows how much space it needs to reserve on the stack for them.

### `std::mem::size_of`

You can verify how much space a type would take on the stack
using the [`std::mem::size_of`](https://doc.rust-lang.org/std/mem/fn.size_of.html) function.

For a `u8`, for example:

```rust
// We'll explain this funny-looking syntax (`::<u8>`) later on.
// Ignore it for now.
assert_eq!(std::mem::size_of::<u8>(), 1);
```

1 makes sense, because a `u8` is 8 bits long, or 1 byte.

[^stack-overflow]: If you have nested function calls, each function pushes its data onto the stack when it's called but
it doesn't pop it off until the innermost function returns.
If you have too many nested function calls, you can run out of stack space—the stack is not infinite!
That's called a [**stack overflow**](https://en.wikipedia.org/wiki/Stack_overflow).
