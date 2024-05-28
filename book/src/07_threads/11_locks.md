# Locks, `Send` and `Arc`

The patching strategy you just implemented has a major drawback: it's racy.\
If two clients send patches for the same ticket roughly at same time, the server will apply them in an arbitrary order.
Whoever enqueues their patch last will overwrite the changes made by the other client.

## Version numbers

We could try to fix this by using a **version number**.\
Each ticket gets assigned a version number upon creation, set to `0`.\
Whenever a client sends a patch, they must include the current version number of the ticket alongside the
desired changes. The server will only apply the patch if the version number matches the one it has stored.

In the scenario described above, the server would reject the second patch, because the version number would
have been incremented by the first patch and thus wouldn't match the one sent by the second client.

This approach is fairly common in distributed systems (e.g. when client and servers don't share memory),
and it is known as **optimistic concurrency control**.\
The idea is that most of the time, conflicts won't happen, so we can optimize for the common case.
You know enough about Rust by now to implement this strategy on your own as a bonus exercise, if you want to.

## Locking

We can also fix the race condition by introducing a **lock**.\
Whenever a client wants to update a ticket, they must first acquire a lock on it. While the lock is active,
no other client can modify the ticket.

Rust's standard library provides two different locking primitives: `Mutex<T>` and `RwLock<T>`.\
Let's start with `Mutex<T>`. It stands for **mut**ual **ex**clusion, and it's the simplest kind of lock:
it allows only one thread to access the data, no matter if it's for reading or writing.

`Mutex<T>` wraps the data it protects, and it's therefore generic over the type of the data.\
You can't access the data directly: the type system forces you to acquire a lock first using either `Mutex::lock` or
`Mutex::try_lock`. The former blocks until the lock is acquired, the latter returns immediately with an error if the lock
can't be acquired.\
Both methods return a guard object that dereferences to the data, allowing you to modify it. The lock is released when
the guard is dropped.

```rust
use std::sync::Mutex;

// An integer protected by a mutex lock
let lock = Mutex::new(0);

// Acquire a lock on the mutex
let mut guard = lock.lock().unwrap();

// Modify the data through the guard,
// leveraging its `Deref` implementation
*guard += 1;

// The lock is released when `data` goes out of scope
// This can be done explicitly by dropping the guard
// or happen implicitly when the guard goes out of scope
drop(guard)
```

## Locking granularity

What should our `Mutex` wrap?\
The simplest option would be the wrap the entire `TicketStore` in a single `Mutex`.\
This would work, but it would severely limit the system's performance: you wouldn't be able to read tickets in parallel,
because every read would have to wait for the lock to be released.\
This is known as **coarse-grained locking**.

It would be better to use **fine-grained locking**, where each ticket is protected by its own lock.
This way, clients can keep working with tickets in parallel, as long as they aren't trying to access the same ticket.

```rust
// The new structure, with a lock for each ticket
struct TicketStore {
    tickets: BTreeMap<TicketId, Mutex<Ticket>>,
}
```

This approach is more efficient, but it has a downside: `TicketStore` has to become **aware** of the multithreaded
nature of the system; up until now, `TicketStore` has been blissfully ignoring the existence of threads.\
Let's go for it anyway.

## Who holds the lock?

For the whole scheme to work, the lock must be passed to the client that wants to modify the ticket.\
The client can then directly modify the ticket (as if they had a `&mut Ticket`) and release the lock when they're done.

This is a bit tricky.\
We can't send a `Mutex<Ticket>` over a channel, because `Mutex` is not `Clone` and
we can't move it out of the `TicketStore`. Could we send the `MutexGuard` instead?

Let's test the idea with a small example:

```rust
use std::thread::spawn;
use std::sync::Mutex;
use std::sync::mpsc::sync_channel;

fn main() {
    let lock = Mutex::new(0);
    let (sender, receiver) = sync_channel(1);
    let guard = lock.lock().unwrap();

    spawn(move || {
        receiver.recv().unwrap();;
    });

    // Try to send the guard over the channel
    // to another thread
    sender.send(guard);
}
```

The compiler is not happy with this code:

```text
error[E0277]: `MutexGuard<'_, i32>` cannot be sent between threads safely
   --> src/main.rs:10:7
    |
10  |   spawn(move || {
    |  _-----_^
    | | |
    | | required by a bound introduced by this call
11  | |     receiver.recv().unwrap();;
12  | | });
    | |_^ `MutexGuard<'_, i32>` cannot be sent between threads safely
    |
    = help: the trait `Send` is not implemented for `MutexGuard<'_, i32>`, which is required by `{closure@src/main.rs:10:7: 10:14}: Send`
    = note: required for `std::sync::mpsc::Receiver<MutexGuard<'_, i32>>` to implement `Send`
note: required because it's used within this closure
```

`MutexGuard<'_, i32>` is not `Send`: what does it mean?

## `Send`

`Send` is a marker trait that indicates that a type can be safely transferred from one thread to another.\
`Send` is also an auto-trait, just like `Sized`; it's automatically implemented (or not implemented) for your type
by the compiler, based on its definition.\
You can also implement `Send` manually for your types, but it requires `unsafe` since you have to guarantee that the
type is indeed safe to send between threads for reasons that the compiler can't automatically verify.

### Channel requirements

`Sender<T>`, `SyncSender<T>` and `Receiver<T>` are `Send` if and only if `T` is `Send`.\
That's because they are used to send values between threads, and if the value itself is not `Send`, it would be
unsafe to send it between threads.

### `MutexGuard`

`MutexGuard` is not `Send` because the underlying operating system primitives that `Mutex` uses to implement
the lock require (on some platforms) that the lock must be released by the same thread that acquired it.\
If we were to send a `MutexGuard` to another thread, the lock would be released by a different thread, which would
lead to undefined behavior.

## Our challenges

Summing it up:

- We can't send a `MutexGuard` over a channel. So we can't lock on the server-side and then modify the ticket on the
  client-side.
- We can send a `Mutex` over a channel because it's `Send` as long as the data it protects is `Send`, which is the
  case for `Ticket`.
  At the same time, we can't move the `Mutex` out of the `TicketStore` nor clone it.

How can we solve this conundrum?\
We need to look at the problem from a different angle.
To lock a `Mutex`, we don't need an owned value. A shared reference is enough, since `Mutex` uses internal mutability:

```rust
impl<T> Mutex<T> {
    // `&self`, not `self`!
    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
        // Implementation details
    }
}
```

It is therefore enough to send a shared reference to the client.\
We can't do that directly, though, because the reference would have to be `'static` and that's not the case.\
In a way, we need an "owned shared reference". It turns out that Rust has a type that fits the bill: `Arc`.

## `Arc` to the rescue

`Arc` stands for **atomic reference counting**.\
`Arc` wraps around a value and keeps track of how many references to the value exist.
When the last reference is dropped, the value is deallocated.\
The value wrapped in an `Arc` is immutable: you can only get shared references to it.

```rust
use std::sync::Arc;

let data: Arc<u32> = Arc::new(0);
let data_clone = Arc::clone(&data);

// `Arc<T>` implements `Deref<T>`, so can convert 
// a `&Arc<T>` to a `&T` using deref coercion
let data_ref: &u32 = &data;
```

If you're having a déjà vu moment, you're right: `Arc` sounds very similar to `Rc`, the reference-counted pointer we
introduced when talking about interior mutability. The difference is thread-safety: `Rc` is not `Send`, while `Arc` is.
It boils down to the way the reference count is implemented: `Rc` uses a "normal" integer, while `Arc` uses an
**atomic** integer, which can be safely shared and modified across threads.

## `Arc<Mutex<T>>`

If we pair `Arc` with `Mutex`, we finally get a type that:

- Can be sent between threads, because:
  - `Arc` is `Send` if `T` is `Send`, and
  - `Mutex` is `Send` if `T` is `Send`.
  - `T` is `Ticket`, which is `Send`.
- Can be cloned, because `Arc` is `Clone` no matter what `T` is.
  Cloning an `Arc` increments the reference count, the data is not copied.
- Can be used to modify the data it wraps, because `Arc` lets you get a shared
  reference to `Mutex<T>` which can in turn be used to acquire a lock.

We have all the pieces we need to implement the locking strategy for our ticket store.

## Further reading

- We won't be covering the details of atomic operations in this course, but you can find more information
  [in the `std` documentation](https://doc.rust-lang.org/std/sync/atomic/index.html) as well as in the
  ["Rust atomics and locks" book](https://marabos.nl/atomics/).
