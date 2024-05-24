# Don't block the runtime

Let's circle back to yield points.\
Unlike threads, **Rust tasks cannot be preempted**.

`tokio` cannot, on its own, decide to pause a task and run another one in its place.
The control goes back to the executor **exclusively** when the task yields—i.e.
when `Future::poll` returns `Poll::Pending` or, in the case of `async fn`, when
you `.await` a future.

This exposes the runtime to a risk: if a task never yields, the runtime will never
be able to run another task. This is called **blocking the runtime**.

## What is blocking?

How long is too long? How much time can a task spend without yielding before it
becomes a problem?

It depends on the runtime, the application, the number of in-flight tasks, and
many other factors. But, as a general rule of thumb, try to spend less than 100
microseconds between yield points.

## Consequences

Blocking the runtime can lead to:

- **Deadlocks**: if the task that's not yielding is waiting for another task to
  complete, and that task is waiting for the first one to yield, you have a deadlock.
  No progress can be made, unless the runtime is able to schedule the other task on
  a different thread.
- **Starvation**: other tasks might not be able to run, or might run after a long
  delay, which can lead to poor performances (e.g. high tail latencies).

## Blocking is not always obvious

Some types of operations should generally be avoided in async code, like:

- Synchronous I/O. You can't predict how long it will take, and it's likely to be
  longer than 100 microseconds.
- Expensive CPU-bound computations.

The latter category is not always obvious though. For example, sorting a vector with
a few elements is not a problem; that evaluation changes if the vector has billions
of entries.

## How to avoid blocking

OK, so how do you avoid blocking the runtime assuming you _must_ perform an operation
that qualifies or risks qualifying as blocking?\
You need to move the work to a different thread. You don't want to use the so-called
runtime threads, the ones used by `tokio` to run tasks.

`tokio` provides a dedicated threadpool for this purpose, called the **blocking pool**.
You can spawn a synchronous operation on the blocking pool using the
`tokio::task::spawn_blocking` function. `spawn_blocking` returns a future that resolves
to the result of the operation when it completes.

```rust
use tokio::task;

fn expensive_computation() -> u64 {
    // [...]
}

async fn run() {
    let handle = task::spawn_blocking(expensive_computation);
    // Do other stuff in the meantime
    let result = handle.await.unwrap();
}
```

The blocking pool is long-lived. `spawn_blocking` should be faster
than creating a new thread directly via `std::thread::spawn`
because the cost of thread initialization is amortized over multiple calls.

## Further reading

- Check out [Alice Ryhl's blog post](https://ryhl.io/blog/async-what-is-blocking/)
  on the topic.
