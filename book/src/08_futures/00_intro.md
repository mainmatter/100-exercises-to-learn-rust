# Async Rust

Threads are not the only way to write concurrent programs in Rust.\
In this chapter we'll explore another approach: **asynchronous programming**.

In particular, you'll get an introduction to:

- The `async`/`.await` keywords, to write asynchronous code effortlessly
- The `Future` trait, to represent computations that may not be complete yet
- `tokio`, the most popular runtime for running asynchronous code
- The cooperative nature of Rust asynchronous model, and how this affects your code
