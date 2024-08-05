# Concise branching

Your solution to the previous exercise probably looks like this:

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        match &self.status {
            Status::InProgress { assigned_to } => assigned_to,
            Status::Done | Status::ToDo => {
                panic!(
                    "Only `In-Progress` tickets can be \
                    assigned to someone"
                )
            }
        }
    }
}
```

You only care about the `Status::InProgress` variant.
Do you really need to match on all the other variants?

New constructs to the rescue!

## `if let`

The `if let` construct allows you to match on a single variant of an enum,
without having to handle all the other variants.

Here's how you can use `if let` to simplify the `assigned_to` method:

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        if let Status::InProgress { assigned_to } = &self.status {
            assigned_to
        } else {
            panic!(
                "Only `In-Progress` tickets can be assigned to someone"
            );
        }
    }
}
```

## `let/else`

If the `else` branch is meant to return early (a panic counts as returning early!),
you can use the `let/else` construct:

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        let Status::InProgress { assigned_to } = &self.status else {
            panic!(
                "Only `In-Progress` tickets can be assigned to someone"
            );
        };
        assigned_to
    }
}
```

It allows you to assign the destructured variable without incurring
any "right drift", i.e. the variable is assigned at the same indentation level
as the code that precedes it.

## Style

Both `if let` and `let/else` are idiomatic Rust constructs.\
Use them as you see fit to improve the readability of your code,
but don't overdo it: `match` is always there when you need it.
