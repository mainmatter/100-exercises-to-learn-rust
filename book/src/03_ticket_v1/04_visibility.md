# Visibility

When you start breaking down your code into multiple modules, you need to start thinking about **visibility**.
Visibility determines which regions of your code (or other people's code) can access a given entity,
be it a struct, a function, a field, etc.

## Private by default

By default, everything in Rust is **private**.\
A private entity can only be accessed:

1. within the same module where it's defined, or
2. by one of its submodules

We've used this extensively in the previous exercises:

- `create_todo_ticket` worked (once you added a `use` statement) because `helpers` is a submodule of the crate root,
  where `Ticket` is defined. Therefore, `create_todo_ticket` can access `Ticket` without any issues even
  though `Ticket` is private.
- All our unit tests are defined in a submodule of the code they're testing, so they can access everything without
  restrictions.

## Visibility modifiers

You can modify the default visibility of an entity using a **visibility modifier**.\
Some common visibility modifiers are:

- `pub`: makes the entity **public**, i.e. accessible from outside the module where it's defined, potentially from
  other crates.
- `pub(crate)`: makes the entity public within the same **crate**, but not outside of it.
- `pub(super)`: makes the entity public within the parent module.
- `pub(in path::to::module)`: makes the entity public within the specified module.

You can use these modifiers on modules, structs, functions, fields, etc.
For example:

```rust
pub struct Configuration {
    pub(crate) version: u32,
    active: bool,
}
```

`Configuration` is public, but you can only access the `version` field from within the same crate.
The `active` field, instead, is private and can only be accessed from within the same module or one of its submodules.
