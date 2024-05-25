# Design review

Let's take a moment to review the journey we've been through.

## Lockless with channel serialization

Our first implementation of a multithreaded ticket store used:

- a single long-lived thread (server), to hold the shared state
- multiple clients sending requests to it via channels from their own threads.

No locking of the state was necessary, since the server was the only one modifying the state. That's because
the "inbox" channel naturally **serialized** incoming requests: the server would process them one by one.\
We've already discussed the limitations of this approach when it comes to patching behaviour, but we didn't
discuss the performance implications of the original design: the server could only process one request at a time,
including reads.

## Fine-grained locking

We then moved to a more sophisticated design, where each ticket was protected by its own lock and
clients could independently decide if they wanted to read or atomically modify a ticket, acquiring the appropriate lock.

This design allows for better parallelism (i.e. multiple clients can read tickets at the same time), but it is
still fundamentally **serial**: the server processes commands one by one. In particular, it hands out locks to clients
one by one.

Could we remove the channels entirely and allow clients to directly access the `TicketStore`, relying exclusively on
locks to synchronize access?

## Removing channels

We have two problems to solve:

- Sharing `TicketStore` across threads
- Synchronizing access to the store

### Sharing `TicketStore` across threads

We want all threads to refer to the same state, otherwise we don't really have a multithreaded system—we're just
running multiple single-threaded systems in parallel.\
We've already encountered this problem when we tried to share a lock across threads: we can use an `Arc`.

### Synchronizing access to the store

There is one interaction that's still lockless thanks to the serialization provided by the channels: inserting
(or removing) a ticket from the store.\
If we remove the channels, we need to introduce (another) lock to synchronize access to the `TicketStore` itself.

If we use a `Mutex`, then it makes no sense to use an additional `RwLock` for each ticket: the `Mutex` will
already serialize access to the entire store, so we wouldn't be able to read tickets in parallel anyway.\
If we use a `RwLock`, instead, we can read tickets in parallel. We just need to pause all reads while inserting
or removing a ticket.

Let's go down this path and see where it leads us.
