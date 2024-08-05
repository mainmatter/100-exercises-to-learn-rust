# Cancellation

What happens when a pending future is dropped?\
The runtime will no longer poll it, therefore it won't make any further progress.
In other words, its execution has been **cancelled**.

In the wild, this often happens when working with timeouts.
For example:

```rust
use tokio::time::timeout;
use tokio::sync::oneshot;
use std::time::Duration;

async fn http_call() {
    // [...]
}

async fn run() {
    // Wrap the future with a `Timeout` set to expire in 10 milliseconds.
    let duration = Duration::from_millis(10);
    if let Err(_) = timeout(duration, http_call()).await {
        println!("Didn't receive a value within 10 ms");
    }
}
```

When the timeout expires, the future returned by `http_call` will be cancelled.
Let's imagine that this is `http_call`'s body:

```rust
use std::net::TcpStream;

async fn http_call() {
    let (stream, _) = TcpStream::connect(/* */).await.unwrap();
    let request: Vec<u8> = /* */;
    stream.write_all(&request).await.unwrap();
}
```

Each yield point becomes a **cancellation point**.\
`http_call` can't be preempted by the runtime, so it can only be discarded after
it has yielded control back to the executor via `.await`.
This applies recursively—e.g. `stream.write_all(&request)` is likely to have multiple
yield points in its implementation. It is perfectly possible to see `http_call` pushing
a _partial_ request before being cancelled, thus dropping the connection and never
finishing transmitting the body.

## Clean up

Rust's cancellation mechanism is quite powerful—it allows the caller to cancel an ongoing task
without needing any form of cooperation from the task itself.\
At the same time, this can be quite dangerous. It may be desirable to perform a
**graceful cancellation**, to ensure that some clean-up tasks are performed
before aborting the operation.

For example, consider this fictional API for a SQL transaction:

```rust
async fn transfer_money(
    connection: SqlConnection,
    payer_id: u64,
    payee_id: u64,
    amount: u64
) -> Result<(), anyhow::Error> {
    let transaction = connection.begin_transaction().await?;
    update_balance(payer_id, amount, &transaction).await?;
    decrease_balance(payee_id, amount, &transaction).await?;
    transaction.commit().await?;
}
```

On cancellation, it'd be ideal to explicitly abort the pending transaction rather
than leaving it hanging.
Rust, unfortunately, doesn't provide a bullet-proof mechanism for this kind of
**asynchronous** clean up operations.

The most common strategy is to rely on the `Drop` trait to schedule the required
clean-up work. This can be by:

- Spawning a new task on the runtime
- Enqueueing a message on a channel
- Spawning a background thread

The optimal choice is contextual.

## Cancelling spawned tasks

When you spawn a task using `tokio::spawn`, you can no longer drop it;
it belongs to the runtime.\
Nonetheless, you can use its `JoinHandle` to cancel it if needed:

```rust
async fn run() {
    let handle = tokio::spawn(/* some async task */);
    // Cancel the spawned task
    handle.abort();
}
```

## Further reading

- Be extremely careful when using `tokio`'s `select!` macro to "race" two different futures.
  Retrying the same task in a loop is dangerous unless you can ensure **cancellation safety**.
  Check out [`select!`'s documentation](https://tokio.rs/tokio/tutorial/select) for more details.\
  If you need to interleave two asynchronous streams of data (e.g. a socket and a channel), prefer using
  [`StreamExt::merge`](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.merge) instead.
- A [`CancellationToken`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html) may be
  preferable to `JoinHandle::abort` in some cases.
