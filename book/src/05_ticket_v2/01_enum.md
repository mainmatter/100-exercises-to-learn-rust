# Enumerations

Based on the validation logic you wrote [in a previous chapter](../03_ticket_v1/02_validation.md),
there are only a few valid statuses for a ticket: `To-Do`, `InProgress` and `Done`.\
This is not obvious if we look at the `status` field in the `Ticket` struct or at the type of the `status`
parameter in the `new` method:

```rust
#[derive(Debug, PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: String
    ) -> Self {
        // [...]
    }
}
```

In both cases we're using `String` to represent the `status` field.
`String` is a very general type—it doesn't immediately convey the information that the `status` field
has a limited set of possible values. Even worse, the caller of `Ticket::new` will only find out **at runtime**
if the status they provided is valid or not.

We can do better than that with **enumerations**.

## `enum`

An enumeration is a type that can have a fixed set of values, called **variants**.\
In Rust, you define an enumeration using the `enum` keyword:

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

`enum`, just like `struct`, defines **a new Rust type**.
