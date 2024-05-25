# `impl Trait`

`TicketStore::to_dos` returns a `Vec<&Ticket>`.\
That signature introduces a new heap allocation every time `to_dos` is called, which may be unnecessary depending
on what the caller needs to do with the result.
It'd be better if `to_dos` returned an iterator instead of a `Vec`, thus empowering the caller to decide whether to
collect the results into a `Vec` or just iterate over them.

That's tricky though!
What's the return type of `to_dos`, as implemented below?

```rust
impl TicketStore {
    pub fn to_dos(&self) -> ??? {
        self.tickets.iter().filter(|t| t.status == Status::ToDo)
    }
}
```

## Unnameable types

The `filter` method returns an instance of `std::iter::Filter`, which has the following definition:

```rust
pub struct Filter<I, P> { /* fields omitted */ }
```

where `I` is the type of the iterator being filtered on and `P` is the predicate used to filter the elements.\
We know that `I` is `std::slice::Iter<'_, Ticket>` in this case, but what about `P`?\
`P` is a closure, an **anonymous function**. As the name suggests, closures don't have a name,
so we can't write them down in our code.

Rust has a solution for this: **impl Trait**.

## `impl Trait`

`impl Trait` is a feature that allows you to return a type without specifying its name.
You just declare what trait(s) the type implements, and Rust figures out the rest.

In this case, we want to return an iterator of references to `Ticket`s:

```rust
impl TicketStore {
    pub fn to_dos(&self) -> impl Iterator<Item = &Ticket> {
        self.tickets.iter().filter(|t| t.status == Status::ToDo)
    }
}
```

That's it!

## Generic?

`impl Trait` in return position is **not** a generic parameter.

Generics are placeholders for types that are filled in by the caller of the function.
A function with a generic parameter is **polymorphic**: it can be called with different types, and the compiler will generate
a different implementation for each type.

That's not the case with `impl Trait`.
The return type of a function with `impl Trait` is **fixed** at compile time, and the compiler will generate
a single implementation for it.
This is why `impl Trait` is also called **opaque return type**: the caller doesn't know the exact type of the return value,
only that it implements the specified trait(s). But the compiler knows the exact type, there is no polymorphism involved.

## RPIT

If you read RFCs or deep-dives about Rust, you might come across the acronym **RPIT**.\
It stands for **"Return Position Impl Trait"** and refers to the use of `impl Trait` in return position.
