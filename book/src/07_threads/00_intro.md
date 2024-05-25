# Intro

One of Rust's big promises is _fearless concurrency_: making it easier to write safe, concurrent programs.
We haven't seen much of that yet. All the work we've done so far has been single-threaded.
Time to change that!

In this chapter we'll make our ticket store multithreaded.\
We'll have the opportunity to touch most of Rust's core concurrency features, including:

- Threads, using the `std::thread` module
- Message passing, using channels
- Shared state, using `Arc`, `Mutex` and `RwLock`
- `Send` and `Sync`, the traits that encode Rust's concurrency guarantees

We'll also discuss various design patterns for multithreaded systems and some of their trade-offs.
