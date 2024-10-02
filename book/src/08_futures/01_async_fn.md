# Asynchronous functions

All the functions and methods you've written so far were eager.\
Nothing happened until you invoked them. But once you did, they ran to
completion: they did **all** their work, and then returned their output.

Sometimes that's undesirable.\
For example, if you're writing an HTTP server, there might be a lot of
**waiting**: waiting for the request body to arrive, waiting for the
database to respond, waiting for a downstream service to reply, etc.

What if you could do something else while you're waiting?\
What if you could choose to give up midway through a computation?\
What if you could choose to prioritise another task over the current one?

That's where **asynchronous functions** come in.

## `async fn`

You use the `async` keyword to define an asynchronous function:

```rust
use tokio::net::TcpListener;

// This function is asynchronous
async fn bind_random() -> TcpListener {
    // [...]
}
```

What happens if you call `bind_random` as you would a regular function?

```rust
fn run() {
    // Invoke `bind_random`
    let listener = bind_random();
    // Now what?
}
```

Nothing happens!\
Rust doesn't start executing `bind_random` when you call it,
not even as a background task (as you might expect based on your experience
with other languages).
Asynchronous functions in Rust are **lazy**: they don't do any work until you
explicitly ask them to.
Using Rust's terminology, we say that `bind_random` returns a **future**, a type
that represents a computation that may complete later. They're called futures
because they implement the `Future` trait, an interface that we'll examine in
detail later on in this chapter.

## `.await`

The most common way to ask an asynchronous function to do some work is to use
the `.await` keyword:

```rust
use tokio::net::TcpListener;

async fn bind_random() -> TcpListener {
    // [...]
}

async fn run() {
    // Invoke `bind_random` and wait for it to complete
    let listener = bind_random().await;
    // Now `listener` is ready
}
```

`.await` doesn't return control to the caller until the asynchronous function
has run to completion—e.g. until the `TcpListener` has been created in the example above.

## Runtimes

If you're puzzled, you're right to be!\
We've just said that the perk of asynchronous functions
is that they don't do **all** their work at once. We then introduced `.await`, which
doesn't return until the asynchronous function has run to completion. Haven't we
just re-introduced the problem we were trying to solve? What's the point?

Not quite! A lot happens behind the scenes when you call `.await`!\
You're yielding control to an **async runtime**, also known as an **async executor**.
Executors are where the magic happens: they are in charge of managing all your
ongoing asynchronous **tasks**. In particular, they balance two different goals:

- **Progress**: they make sure that tasks make progress whenever they can.
- **Efficiency**: if a task is waiting for something, they try to make sure that
  another task can run in the meantime, fully utilising the available resources.

### No default runtime

Rust is fairly unique in its approach to asynchronous programing: there is
no default runtime. The standard library doesn't ship with one. You need to
bring your own!

In most cases, you'll choose one of the options available in the ecosystem.
Some runtimes are designed to be broadly applicable, a solid option for most applications.
`tokio` and `async-std` belong to this category. Other runtimes are optimised for
specific use cases—e.g. `embassy` for embedded systems.

Throughout this course we'll rely on `tokio`, the most popular runtime for general-purpose
asynchronous programming in Rust.

### `#[tokio::main]`

The entrypoint of your executable, the `main` function, must be a synchronous function.
That's where you're supposed to set up and launch your chosen async runtime.

Most runtimes provide a macro to make this easier. For `tokio`, it's `tokio::main`:

```rust
#[tokio::main]
async fn main() {
    // Your async code goes here
}
```

which expands to:

```rust
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(
        // Your async function goes here
        // [...]
    );
}
```

### `#[tokio::test]`

The same goes for tests: they must be synchronous functions.\
Each test function is run in its own thread, and you're responsible for
setting up and launching an async runtime if you need to run async code
in your tests.\
`tokio` provides a `#[tokio::test]` macro to make this easier:

```rust
#[tokio::test]
async fn my_test() {
    // Your async test code goes here
}
```
