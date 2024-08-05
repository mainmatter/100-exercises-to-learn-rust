# Encapsulation

Now that we have a basic understanding of modules and visibility, let's circle back to **encapsulation**.\
Encapsulation is the practice of hiding the internal representation of an object. It is most commonly
used to enforce some **invariants** on the object's state.

Going back to our `Ticket` struct:

```rust
struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

If all fields are made public, there is no encapsulation.\
You must assume that the fields can be modified at any time, set to any value that's allowed by
their type. You can't rule out that a ticket might have an empty title or a status
that doesn't make sense.

To enforce stricter rules, we must keep the fields private[^newtype].
We can then provide public methods to interact with a `Ticket` instance.
Those public methods will have the responsibility of upholding our invariants (e.g. a title must not be empty).

If at least one field is private it is no longer possible to create a `Ticket` instance directly using the struct
instantiation syntax:

```rust
// This won't work!
let ticket = Ticket {
    title: "Build a ticket system".into(),
    description: "A Kanban board".into(),
    status: "Open".into()
};
```

You've seen this in action in the previous exercise on visibility.\
We now need to provide one or more public **constructors**—i.e. static methods or functions that can be used
from outside the module to create a new instance of the struct.\
Luckily enough we already have one: `Ticket::new`, as implemented in [a previous exercise](02_validation.md).

## Accessor methods

In summary:

- All `Ticket` fields are private
- We provide a public constructor, `Ticket::new`, that enforces our validation rules on creation

That's a good start, but it's not enough: apart from creating a `Ticket`, we also need to interact with it.
But how can we access the fields if they're private?

We need to provide **accessor methods**.\
Accessor methods are public methods that allow you to read the value of a private field (or fields) of a struct.

Rust doesn't have a built-in way to generate accessor methods for you, like some other languages do.
You have to write them yourself—they're just regular methods.

[^newtype]: Or refine their type, a technique we'll explore [later on](../05_ticket_v2/15_outro.md).
