# Threads

Before we start writing multithreaded code, let's take a step back and talk about what threads are
and why we might want to use them.

## What is a thread?

A **thread** is an execution context managed by the underlying operating system.\
Each thread has its own stack and instruction pointer.

A single **process** can manage multiple threads.
These threads share the same memory space, which means they can access the same data.

Threads are a **logical** construct. In the end, you can only run one set of instructions
at a time on a CPU core, the **physical** execution unit.\
Since there can be many more threads than there are CPU cores, the operating system's
**scheduler** is in charge of deciding which thread to run at any given time,
partitioning CPU time among them to maximize throughput and responsiveness.

## `main`

When a Rust program starts, it runs on a single thread, the **main thread**.\
This thread is created by the operating system and is responsible for running the `main`
function.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}
```

## `std::thread`

Rust's standard library provides a module, `std::thread`, that allows you to create
and manage threads.

### `spawn`

You can use `std::thread::spawn` to create new threads and execute code on them.

For example:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("Hello from a thread!");
        }
    });
    
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}
```

If you execute this program on the [Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=afedf7062298ca8f5a248bc551062eaa)
you'll see that the main thread and the spawned thread run concurrently.\
Each thread makes progress independently of the other.

### Process termination

When the main thread finishes, the overall process will exit.\
A spawned thread will continue running until it finishes or the main thread finishes.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("Hello from a thread!");
        }
    });

    thread::sleep(Duration::from_secs(5));
}
```

In the example above, you can expect to see the message "Hello from a thread!" printed roughly five times.\
Then the main thread will finish (when the `sleep` call returns), and the spawned thread will be terminated
since the overall process exits.

### `join`

You can also wait for a spawned thread to finish by calling the `join` method on the `JoinHandle` that `spawn` returns.

```rust
use std::thread;
fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}
```

In this example, the main thread will wait for the spawned thread to finish before exiting.\
This introduces a form of **synchronization** between the two threads: you're guaranteed to see the message
"Hello from a thread!" printed before the program exits, because the main thread won't exit
until the spawned thread has finished.
