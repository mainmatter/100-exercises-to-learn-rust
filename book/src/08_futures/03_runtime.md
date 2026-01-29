# Runtime architecture

So far we've been talking about async runtimes as an abstract concept.
Let's dig a bit deeper into the way they are implemented—as you'll see soon enough,
it has an impact on our code.

## Flavors

`tokio` ships two different runtime _flavors_.

You can configure your runtime via `tokio::runtime::Builder`:

- `Builder::new_multi_thread` gives you a **multithreaded `tokio` runtime**
- `Builder::new_current_thread` will instead rely on the **current thread** for execution.

`#[tokio::main]` returns a multithreaded runtime by default, while
`#[tokio::test]` uses a current thread runtime out of the box.

### Current thread runtime

The current-thread runtime, as the name implies, relies exclusively on the OS thread
it was launched on to schedule and execute tasks.\
When using the current-thread runtime, you have **concurrency** but no **parallelism**:
asynchronous tasks will be interleaved, but there will always be at most one task running
at any given time.

### Multithreaded runtime

When using the multithreaded runtime, instead, there can be up to `N` tasks running
_in parallel_ at any given time, where `N` is the number of threads used by the
runtime. By default, `N` matches the number of available CPU cores.

There's more: `tokio` performs **work-stealing**.\
If a thread is idle, it won't wait around: it'll try to find a new task that's ready for
execution, either from a global queue or by stealing it from the local queue of another
thread.\
Work-stealing can have significant performance benefits, especially on tail latencies,
whenever your application is dealing with workloads that are not perfectly balanced
across threads.

## Implications

`tokio::spawn` is flavor-agnostic: it'll work no matter if you're running on the multithreaded
or current-thread runtime. The downside is that the signature assumes the worst case
(i.e. multithreaded) and is constrained accordingly:

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{ /* */ }
```

Let's ignore the `Future` trait for now to focus on the rest.\
`spawn` is asking all its inputs to be `Send` and have a `'static` lifetime.

The `'static` constraint follows the same rationale of the `'static` constraint
on `std::thread::spawn`: the spawned task may outlive the context it was spawned
from, therefore it shouldn't depend on any local data that may be de-allocated
after the spawning context is destroyed.

```rust
fn spawner() {
    let v = vec![1, 2, 3];
    // This won't work, since `&v` doesn't
    // live long enough.
    tokio::spawn(async { 
        for x in &v {
            println!("{x}")
        }
    })
}
```

`Send`, on the other hand, is a direct consequence of `tokio`'s work-stealing strategy:
a task that was spawned on thread `A` may end up being moved to thread `B` if that's idle,
thus requiring a `Send` bound since we're crossing thread boundaries.

```rust
fn spawner(input: Rc<u64>) {
    // This won't work either, because
    // `Rc` isn't `Send`.
    tokio::spawn(async move {
        println!("{}", input);
    })
}
```
