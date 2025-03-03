# Variants can hold data

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

Our `Status` enum is what's usually called a **C-style enum**.\
Each variant is a simple label, a bit like a named constant. You can find this kind of enum in many programming
languages, like C, C++, Java, C#, Python, etc.

Rust enums can go further though. We can **attach data to each variant**.

## Variants

Let's say that we want to store the name of the person who's currently working on a ticket.\
We would only have this information if the ticket is in progress. It wouldn't be there for a to-do ticket or
a done ticket.
We can model this by attaching a `String` field to the `InProgress` variant:

```rust
enum Status {
    ToDo,
    InProgress {
        assigned_to: String,
    },
    Done,
}
```

`InProgress` is now a **struct-like variant**.\
The syntax mirrors, in fact, the one we used to define a struct—it's just "inlined" inside the enum, as a variant.

## Accessing variant data

If we try to access `assigned_to` on a `Status` instance,

```rust
let status: Status = /* */;

// This won't compile
println!("Assigned to: {}", status.assigned_to);
```

the compiler will stop us:

```text
error[E0609]: no field `assigned_to` on type `Status`
 --> src/main.rs:5:40
  |
5 |     println!("Assigned to: {}", status.assigned_to);
  |                                        ^^^^^^^^^^^ unknown field
```

`assigned_to` is **variant-specific**, it's not available on all `Status` instances.\
To access `assigned_to`, we need to use **pattern matching**:

```rust
match status {
    Status::InProgress { assigned_to } => {
        println!("Assigned to: {}", assigned_to);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```

## Bindings

In the match pattern `Status::InProgress { assigned_to }`, `assigned_to` is a **binding**.\
We're **destructuring** the `Status::InProgress` variant and binding the `assigned_to` field to
a new variable, also named `assigned_to`.\
If we wanted, we could bind the field to a different variable name:

```rust
match status {
    Status::InProgress { assigned_to: person } => {
        println!("Assigned to: {}", person);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```
