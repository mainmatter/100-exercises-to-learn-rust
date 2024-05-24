# Ticket ids

Let's think again about our ticket management system.\
Our ticket model right now looks like this:

```rust
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

One thing is missing here: an **identifier** to uniquely identify a ticket.\
That identifier should be unique for each ticket. That can be guaranteed by generating it automatically when
a new ticket is created.

## Refining the model

Where should the id be stored?\
We could add a new field to the `Ticket` struct:

```rust
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

But we don't know the id before creating the ticket. So it can't be there from the get-go.\
It'd have to be optional:

```rust
pub struct Ticket {
    pub id: Option<TicketId>,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

That's also not ideal—we'd have to handle the `None` case every single time we retrieve a ticket from the store,
even though we know that the id should always be there once the ticket has been created.

The best solution is to have two different ticket **states**, represented by two separate types:
a `TicketDraft` and a `Ticket`:

```rust
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription
}

pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

A `TicketDraft` is a ticket that hasn't been created yet. It doesn't have an id, and it doesn't have a status.\
A `Ticket` is a ticket that has been created. It has an id and a status.\
Since each field in `TicketDraft` and `Ticket` embeds its own constraints, we don't have to duplicate logic
across the two types.
