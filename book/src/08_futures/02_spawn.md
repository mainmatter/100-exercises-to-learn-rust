# Spawning tasks

Your solution to the previous exercise should look something like this:

```rust
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        let (mut reader, mut writer) = socket.split();
        tokio::io::copy(&mut reader, &mut writer).await?;
    }
}
```

This is not bad!\
If a long time passes between two incoming connections, the `echo` function will be idle
(since `TcpListener::accept` is an asynchronous function), thus allowing the executor
to run other tasks in the meantime.

But how can we actually have multiple tasks running concurrently?\
If we always run our asynchronous functions until completion (by using `.await`), we'll never
have more than one task running at a time.

This is where the `tokio::spawn` function comes in.

## `tokio::spawn`

`tokio::spawn` allows you to hand off a task to the executor, **without waiting for it to complete**.\
Whenever you invoke `tokio::spawn`, you're telling `tokio` to continue running
the spawned task, in the background, **concurrently** with the task that spawned it.

Here's how you can use it to process multiple connections concurrently:

```rust
use tokio::net::TcpListener;

pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        // Spawn a background task to handle the connection
        // thus allowing the main task to immediately start 
        // accepting new connections
        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();
            tokio::io::copy(&mut reader, &mut writer).await?;
        });
    }
}
```

### Asynchronous blocks

In this example, we've passed an **asynchronous block** to `tokio::spawn`: `async move { /* */ }`
Asynchronous blocks are a quick way to mark a region of code as asynchronous without having
to define a separate async function.

### `JoinHandle`

`tokio::spawn` returns a `JoinHandle`.\
You can use `JoinHandle` to `.await` the background task, in the same way
we used `join` for spawned threads.

```rust
pub async fn run() {
    // Spawn a background task to ship telemetry data
    // to a remote server
    let handle = tokio::spawn(emit_telemetry());
    // In the meantime, do some other useful work
    do_work().await;
    // But don't return to the caller until 
    // the telemetry data has been successfully delivered
    handle.await;
}

pub async fn emit_telemetry() {
    // [...]
}

pub async fn do_work() {
    // [...]
}
```

### Panic boundary

If a task spawned with `tokio::spawn` panics, the panic will be caught by the executor.\
If you don't `.await` the corresponding `JoinHandle`, the panic won't be propagated to the spawner.
Even if you do `.await` the `JoinHandle`, the panic won't be propagated automatically.
Awaiting a `JoinHandle` returns a `Result`, with [`JoinError`](https://docs.rs/tokio/latest/tokio/task/struct.JoinError.html)
as its error type. You can then check if the task panicked by calling `JoinError::is_panic` and
choose what to do with the panic—either log it, ignore it, or propagate it.

```rust
use tokio::task::JoinError;

pub async fn run() {
    let handle = tokio::spawn(work());
    if let Err(e) = handle.await {
        if let Ok(reason) = e.try_into_panic() {
            // The task has panicked
            // We resume unwinding the panic,
            // thus propagating it to the current task
            panic::resume_unwind(reason);
        }
    }
}

pub async fn work() {
    // [...]
}
```

### `std::thread::spawn` vs `tokio::spawn`

You can think of `tokio::spawn` as the asynchronous sibling of `std::spawn::thread`.

Notice a key difference: with `std::thread::spawn`, you're delegating control to the OS scheduler.
You're not in control of how threads are scheduled.

With `tokio::spawn`, you're delegating to an async executor that runs entirely in
user space. The underlying OS scheduler is not involved in the decision of which task
to run next. We're in charge of that decision now, via the executor we chose to use.
