# Heap

The stack is great, but it can't solve all our problems. What about data whose size is not known at compile time?
Collections, strings, and other dynamically-sized data cannot be (entirely) stack-allocated.
That's where the **heap** comes in.

## Heap allocations

You can visualize the heap as a big chunk of memory—a huge array, if you will.  
Whenever you need to store data on the heap, you ask a special program, the **allocator**, to reserve for you
a subset of the heap. We call this interaction (and the memory you reserved) a **heap allocation**.
If the allocation succeeds, the allocator will give you a **pointer** to the start of the reserved block.

## No automatic de-allocation

The heap is structured quite differently from the stack.  
Heap allocations are not contiguous, they can be located anywhere inside the heap.

```
+---+---+---+---+---+---+-...-+-...-+---+---+---+---+---+---+---+
|  Allocation 1 | Free  | ... | ... |  Allocation N |    Free   |
+---+---+---+---+---+---+ ... + ... +---+---+---+---+---+---+---+
```

It's the allocator's job to keep track of which parts of the heap are in use and which are free.
The allocator won't automatically free the memory you allocated, though: you need to be deliberate about it,
calling the allocator again to **free** the memory you no longer need.

## Performance

The heap's flexibility comes at a cost: heap allocations are **slower** than stack allocations.
There's a lot more bookkeeping involved!  
If you read articles about performance optimization you'll often be advised to minimize heap allocations 
and prefer stack-allocated data whenever possible.

## `String`'s memory layout

When you create a local variable of type `String`, 
Rust is forced to allocate on the heap[^empty]: it doesn't know in advance how much text you're going to put in it, 
so it can't reserve the right amount of space on the stack.  
But a `String` is not _entirely_ heap-allocated, it also keeps some data on the stack. In particular:

- The **pointer** to the heap region you reserved.
- The **length** of the string, i.e. how many bytes are in the string.
- The **capacity** of the string, i.e. how many bytes have been reserved on the heap.

Let's look at an example to understand this better:

```rust
let mut s = String::with_capacity(5);
```

If you run this code, memory will be laid out like this:

```
      +---------+--------+----------+
Stack | pointer | length | capacity | 
      |  |      |   0    |    5     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+---+---+
Heap:  | ? | ? | ? | ? | ? |
       +---+---+---+---+---+
```

We asked for a `String` that can hold up to 5 bytes of text.  
`String::with_capacity` goes to the allocator and asks for 5 bytes of heap memory. The allocator returns 
a pointer to the start of that memory block.  
The `String` is empty, though. On the stack, we keep track of this information by distinguishing between 
the length and the capacity: this `String` can hold up to 5 bytes, but it currently holds 0 bytes of 
actual text.

If you push some text into the `String`, the situation will change:

```rust
s.push_str("Hey");
```

```
      +---------+--------+----------+
Stack | pointer | length | capacity |
      |  |      |   3    |    5     |
      +--|  ----+--------+----------+
         |
         |
         v
       +---+---+---+---+---+
Heap:  | H | e | y | ? | ? |
       +---+---+---+---+---+
```

`s` now holds 3 bytes of text. Its length is updated to 3, but capacity remains 5.
Three of the five bytes on the heap are used to store the characters `H`, `e`, and `y`.

### `usize`

How much space do we need to store pointer, length and capacity on the stack?  
It depends on the **architecture** of the machine you're running on.

Every memory location on your machine has an [**address**](https://en.wikipedia.org/wiki/Memory_address), commonly
represented as an unsigned integer.
Depending on the maximum size of the address space (i.e. how much memory your machine can address), 
this integer can have a different size. Most modern machines use either a 32-bit or a 64-bit address space.  

Rust abstracts away these architecture-specific details by providing the `usize` type:
an unsigned integer that's as big as the number of bytes needed to address memory on your machine.
On a 32-bit machine, `usize` is equivalent to `u32`. On a 64-bit machine, it matches `u64`.

Capacity, length and pointers are all represented as `usize`s in Rust[^equivalence].  

### No `std::mem::size_of` for the heap

`std::mem::size_of` returns the amount of space a type would take on the stack, 
which is also known as the **size of the type**.  

> What about the memory buffer that `String` is managing on the heap? Isn't that
> part of the size of `String`?

No!  
That heap allocation is a **resource** that `String` is managing. 
It's not considered to be part of the `String` type by the compiler.  

`std::mem::size_of` doesn't know (or care) about additional heap-allocated data 
that a type might manage or refer to via pointers, as is the case with `String`,
therefore it doesn't track its size.

Unfortunately there is no equivalent of `std::mem::size_of` to measure the amount of
heap memory that a certain value is allocating at runtime. Some types might
provide methods to inspect their heap usage (e.g. `String`'s `capacity` method),
but there is no general-purpose "API" to retrieve runtime heap usage in Rust.  
You can, however, use a memory profiler tool (e.g. [DHAT](https://valgrind.org/docs/manual/dh-manual.html)
or [a custom allocator](https://docs.rs/dhat/latest/dhat/)) to inspect the heap usage of your program.

## References

- The exercise for this section is located in `exercises/03_ticket_v1/09_heap`

[^empty]: `std` doesn't allocate if you create an **empty** `String` (i.e. `String::new()`).
  Heap memory will be reserved when you push data into it for the first time.

[^equivalence]: The size of a pointer depends on the operating system too.
  In certain environments, a pointer is **larger** than a memory address (e.g. [CHERI](https://blog.acolyer.org/2019/05/28/cheri-abi/)). 
  Rust makes the simplifying assumption that pointers are the same size as memory addresses,
  which is true for most modern systems you're likely to encounter.