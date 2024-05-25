# Channels

All our spawned threads have been fairly short-lived so far.\
Get some input, run a computation, return the result, shut down.

For our ticket management system, we want to do something different:
a client-server architecture.

We will have **one long-running server thread**, responsible for managing
our state, the stored tickets.

We will then have **multiple client threads**.\
Each client will be able to send **commands** and **queries** to
the stateful thread, in order to change its state (e.g. add a new ticket)
or retrieve information (e.g. get the status of a ticket).\
Client threads will run concurrently.

## Communication

So far we've only had very limited parent-child communication:

- The spawned thread borrowed/consumed data from the parent context
- The spawned thread returned data to the parent when joined

This isn't enough for a client-server design.\
Clients need to be able to send and receive data from the server thread
_after_ it has been launched.

We can solve the issue using **channels**.

## Channels

Rust's standard library provides **multi-producer, single-consumer** (mpsc) channels
in its `std::sync::mpsc` module.\
There are two channel flavours: bounded and unbounded. We'll stick to the unbounded
version for now, but we'll discuss the pros and cons later on.

Channel creation looks like this:

```rust
use std::sync::mpsc::channel;

let (sender, receiver) = channel();
```

You get a sender and a receiver.\
You call `send` on the sender to push data into the channel.\
You call `recv` on the receiver to pull data from the channel.

### Multiple senders

`Sender` is clonable: we can create multiple senders (e.g. one for
each client thread) and they will all push data into the same channel.

`Receiver`, instead, is not clonable: there can only be a single receiver
for a given channel.

That's what **mpsc** (multi-producer single-consumer) stands for!

### Message type

Both `Sender` and `Receiver` are generic over a type parameter `T`.\
That's the type of the _messages_ that can travel on our channel.

It could be a `u64`, a struct, an enum, etc.

### Errors

Both `send` and `recv` can fail.\
`send` returns an error if the receiver has been dropped.\
`recv` returns an error if all senders have been dropped and the channel is empty.

In other words, `send` and `recv` error when the channel is effectively closed.
