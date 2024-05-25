# Update operations

So far we've implemented only insertion and retrieval operations.\
Let's see how we can expand the system to provide an update operation.

## Legacy updates

In the non-threaded version of the system, updates were fairly straightforward: `TicketStore` exposed a
`get_mut` method that allowed the caller to obtain a mutable reference to a ticket, and then modify it.

## Multithreaded updates

The same strategy won't work in the current multi-threaded version,
because the mutable reference would have to be sent over a channel. The borrow checker would
stop us, because `&mut Ticket` doesn't satisfy the `'static` lifetime requirement of `SyncSender::send`.

There are a few ways to work around this limitation. We'll explore a few of them in the following exercises.

### Patching

We can't send a `&mut Ticket` over a channel, therefore we can't mutate on the client-side.\
Can we mutate on the server-side?

We can, if we tell the server what needs to be changed. In other words, if we send a **patch** to the server:

```rust
struct TicketPatch {
    id: TicketId,
    title: Option<TicketTitle>,
    description: Option<TicketDescription>,
    status: Option<TicketStatus>,
}
```

The `id` field is mandatory, since it's required to identify the ticket that needs to be updated.\
All other fields are optional:

- If a field is `None`, it means that the field should not be changed.
- If a field is `Some(value)`, it means that the field should be changed to `value`.
