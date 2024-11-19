# Leaking data

The main concern around passing references to spawned threads is use-after-free bugs:
accessing data using a pointer to a memory region that's already been freed/de-allocated.\
If you're working with heap-allocated data, you can avoid the issue by
telling Rust that you'll never reclaim that memory: you choose to **leak memory**,
intentionally.

This can be done, for example, using the `Box::leak` method from Rust's standard library:

```rust
// Allocate a `u32` on the heap, by wrapping it in a `Box`.
let x = Box::new(41u32);
// Tell Rust that you'll never free that heap allocation
// using `Box::leak`. You can thus get back a 'static reference.
let static_ref: &'static mut u32 = Box::leak(x);
```

## Data leakage is process-scoped

Leaking data is dangerous: if you keep leaking memory, you'll eventually
run out and crash with an out-of-memory error.

```rust
// If you leave this running for a while, 
// it'll eventually use all the available memory.
fn oom_trigger() {
    loop {
        let v: Vec<usize> = Vec::with_capacity(1024);
        v.leak();
    }
}
```

At the same time, memory leaked via `leak` method is not truly forgotten.\
The operating system can map each memory region to the process responsible for it.
When the process exits, the operating system will reclaim that memory.

Keeping this in mind, it can be OK to leak memory when:

- The amount of memory you need to leak is bounded/known upfront, or
- Your process is short-lived and you're confident you won't exhaust
  all the available memory before it exits

"Let the OS deal with it" is a perfectly valid memory management strategy
if your usecase allows for it.
