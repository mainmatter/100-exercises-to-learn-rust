# `Sync`

Before we wrap up this chapter, let's talk about another key trait in Rust's standard library: `Sync`.

`Sync` is an auto trait, just like `Send`.\
It is automatically implemented by all types that can be safely **shared** between threads.

In other words: `T` is Sync if `&T` is `Send`.

## `T: Sync` doesn't imply `T: Send`

It's important to note that `T` can be `Sync` without being `Send`.\
For example: `MutexGuard` is not `Send`, but it is `Sync`.

It isn't `Send` because the lock must be released on the same thread that acquired it, therefore we don't
want `MutexGuard` to be dropped on a different thread.\
But it is `Sync`, because giving a `&MutexGuard` to another thread has no impact on where the lock is released.

## `T: Send` doesn't imply `T: Sync`

The opposite is also true: `T` can be `Send` without being `Sync`.\
For example: `RefCell<T>` is `Send` (if `T` is `Send`), but it is not `Sync`.

`RefCell<T>` performs runtime borrow checking, but the counters it uses to track borrows are not thread-safe.
Therefore, having multiple threads holding a `&RefCell` would lead to a data race, with potentially
multiple threads obtaining mutable references to the same data. Hence `RefCell` is not `Sync`.\
`Send` is fine, instead, because when we send a `RefCell` to another thread we're not
leaving behind any references to the data it contains, hence no risk of concurrent mutable access.
