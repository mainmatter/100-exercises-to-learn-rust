# Async-aware primitives

If you browse `tokio`'s documentation, you'll notice that it provides a lot of types
that "mirror" the ones in the standard library, but with an asynchronous twist:
locks, channels, timers, and more.

When working in an asynchronous context, you should prefer these asynchronous alternatives
to their synchronous counterparts.

To understand why, let's take a look at `Mutex`, the mutually exclusive lock we explored
in the previous chapter.

## Case study: `Mutex`

Let's look at a simple example:

```rust
use std::sync::{Arc, Mutex};

async fn run(m: Arc<Mutex<Vec<u64>>>) {
    let guard = m.lock().unwrap();
    http_call(&guard).await;
    println!("Sent {:?} to the server", &guard);
    // `guard` is dropped here
}

/// Use `v` as the body of an HTTP call.
async fn http_call(v: &[u64]) {
  // [...]
}
```

### `std::sync::MutexGuard` and yield points

This code will compile, but it's dangerous.

We try to acquire a lock over a `Mutex` from `std` in an asynchronous context.
We then hold on to the resulting `MutexGuard` across a yield point (the `.await` on
`http_call`).

Let's imagine that there are two tasks executing `run`, concurrently, on a single-threaded
runtime. We observe the following sequence of scheduling events:

```text
     Task A          Task B
        | 
  Acquire lock
Yields to runtime
        | 
        +--------------+
                       |
             Tries to acquire lock
```

We have a deadlock. Task B will never manage to acquire the lock, because the lock
is currently held by task A, which has yielded to the runtime before releasing the
lock and won't be scheduled again because the runtime cannot preempt task B.

### `tokio::sync::Mutex`

You can solve the issue by switching to `tokio::sync::Mutex`:

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

async fn run(m: Arc<Mutex<Vec<u64>>>) {
    let guard = m.lock().await;
    http_call(&guard).await;
    println!("Sent {:?} to the server", &guard);
    // `guard` is dropped here
}
```

Acquiring the lock is now an asynchronous operation, which yields back to the runtime
if it can't make progress.\
Going back to the previous scenario, the following would happen:

```text
       Task A          Task B
          | 
  Acquires the lock
  Starts `http_call`
  Yields to runtime
          | 
          +--------------+
                         |
             Tries to acquire the lock
              Cannot acquire the lock
                 Yields to runtime
                         |
          +--------------+
          |
`http_call` completes      
  Releases the lock
   Yield to runtime
          |
          +--------------+
                         |
                 Acquires the lock
                       [...]
```

All good!

### Multithreaded won't save you

We've used a single-threaded runtime as the execution context in our
previous example, but the same risk persists even when using a multithreaded
runtime.\
The only difference is in the number of concurrent tasks required to create the deadlock:
in a single-threaded runtime, 2 are enough; in a multithreaded runtime, we
would need `N+1` tasks, where `N` is the number of runtime threads.

### Downsides

Having an async-aware `Mutex` comes with a performance penalty.\
If you're confident that the lock isn't under significant contention
_and_ you're careful to never hold it across a yield point, you can
still use `std::sync::Mutex` in an asynchronous context.

But weigh the performance benefit against the liveness risk you
will incur.

## Other primitives

We used `Mutex` as an example, but the same applies to `RwLock`, semaphores, etc.\
Prefer async-aware versions when working in an asynchronous context to minimise
the risk of issues.
