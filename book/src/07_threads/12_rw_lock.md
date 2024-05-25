# Readers and writers

Our new `TicketStore` works, but its read performance is not great: there can only be one client at a time
reading a specific ticket, because `Mutex<T>` doesn't distinguish between readers and writers.

We can solve the issue by using a different locking primitive: `RwLock<T>`.\
`RwLock<T>` stands for **read-write lock**. It allows **multiple readers** to access the data simultaneously,
but only one writer at a time.

`RwLock<T>` has two methods to acquire a lock: `read` and `write`.\
`read` returns a guard that allows you to read the data, while `write` returns a guard that allows you to modify it.

```rust
use std::sync::RwLock;

// An integer protected by a read-write lock
let lock = RwLock::new(0);

// Acquire a read lock on the RwLock
let guard1 = lock.read().unwrap();

// Acquire a **second** read lock
// while the first one is still active
let guard2 = lock.read().unwrap();
```

## Trade-offs

On the surface, `RwLock<T>` seems like a no-brainer: it provides a superset of the functionality of `Mutex<T>`.
Why would you ever use `Mutex<T>` if you can use `RwLock<T>` instead?

There are two key reasons:

- Locking a `RwLock<T>` is more expensive than locking a `Mutex<T>`.\
  This is because `RwLock<T>` has to keep track of the number of active readers and writers, while `Mutex<T>`
  only has to keep track of whether the lock is held or not.
  This performance overhead is not an issue if there are more readers than writers, but if the workload
  is write-heavy `Mutex<T>` might be a better choice.
- `RwLock<T>` can cause **writer starvation**.\
  If there are always readers waiting to acquire the lock, writers might never get a chance to run.\
  `RwLock<T>` doesn't provide any guarantees about the order in which readers and writers are granted access to the lock.
  It depends on the policy implemented by the underlying OS, which might not be fair to writers.

In our case, we can expect the workload to be read-heavy (since most clients will be reading tickets, not modifying them),
so `RwLock<T>` is a good choice.
