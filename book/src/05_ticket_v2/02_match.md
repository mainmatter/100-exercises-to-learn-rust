# `match`

You may be wondering—what can you actually **do** with an enum?\
The most common operation is to **match** on it.

```rust
enum Status {
    ToDo,
    InProgress,
    Done
}

impl Status {
    fn is_done(&self) -> bool {
        match self {
            Status::Done => true,
            // The `|` operator lets you match multiple patterns.
            // It reads as "either `Status::ToDo` or `Status::InProgress`".
            Status::InProgress | Status::ToDo => false
        }
    }
}
```

A `match` statement that lets you compare a Rust value against a series of **patterns**.\
You can think of it as a type-level `if`. If `status` is a `Done` variant, execute the first block;
if it's a `InProgress` or `ToDo` variant, execute the second block.

## Exhaustiveness

There's one key detail here: `match` is **exhaustive**. You must handle all enum variants.\
If you forget to handle a variant, Rust will stop you **at compile-time** with an error.

E.g. if we forget to handle the `ToDo` variant:

```rust
match self {
    Status::Done => true,
    Status::InProgress => false,
}
```

the compiler will complain:

```text
error[E0004]: non-exhaustive patterns: `ToDo` not covered
 --> src/main.rs:5:9
  |
5 |     match status {
  |     ^^^^^^^^^^^^ pattern `ToDo` not covered
```

This is a big deal!\
Codebases evolve over time—you might add a new status down the line, e.g. `Blocked`. The Rust compiler
will emit an error for every single `match` statement that's missing logic for the new variant.
That's why Rust developers often sing the praises of "compiler-driven refactoring"—the compiler tells you
what to do next, you just have to fix what it reports.

## Catch-all

If you don't care about one or more variants, you can use the `_` pattern as a catch-all:

```rust
match status {
    Status::Done => true,
    _ => false
}
```

The `_` pattern matches anything that wasn't matched by the previous patterns.

<div class="warning">
  The convenience of `_` comes at a cost: the compiler can no longer help you when the enum changes. In the above example, where a `Blocked` variant is added to `Status`, this `match` keeps compiling as the new `Blocked` variant is silently handled by the `_` arm, whether or not that's the behavior you want.

When correctness matters, spell out every variant, even if several arms share the same logic
(e.g. `Status::ToDo | Status::InProgress => false`). That way, each new variant forces a deliberate decision.

</div>
