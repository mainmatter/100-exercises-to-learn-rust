# Derive macros

Implementing `PartialEq` for `Ticket` was a bit tedious, wasn't it?
You had to manually compare each field of the struct.

## Destructuring syntax

Furthermore, the implementation is brittle: if the struct definition changes
(e.g. a new field is added), you have to remember to update the `PartialEq` implementation.

You can mitigate the risk by **destructuring** the struct into its fields:

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        // [...]
    }
}
```

If the definition of `Ticket` changes, the compiler will error out, complaining that your
destructuring is no longer exhaustive.\
You can also rename struct fields, to avoid variable shadowing:

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        let Ticket {
            title: other_title,
            description: other_description,
            status: other_status,
        } = other;
        // [...]
    }
}
```

Destructuring is a useful pattern to have in your toolkit, but
there's an even more convenient way to do this: **derive macros**.

## Macros

You've already encountered a few macros in past exercises:

- `assert_eq!` and `assert!`, in the test cases
- `println!`, to print to the console

Rust macros are **code generators**.\
They generate new Rust code based on the input you provide, and that generated code is then compiled alongside
the rest of your program. Some macros are built into Rust's standard library, but you can also
write your own. We won't be creating our own macro in this course, but you can find some useful
pointers in the ["Further reading" section](#further-reading).

### Inspection

Some IDEs let you expand a macro to inspect the generated code. If that's not possible, you can use
[`cargo-expand`](https://github.com/dtolnay/cargo-expand).

### Derive macros

A **derive macro** is a particular flavour of Rust macro. It is specified as an **attribute** on top of a struct.

```rust
#[derive(PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

Derive macros are used to automate the implementation of common (and "obvious") traits for custom types.
In the example above, the `PartialEq` trait is automatically implemented for `Ticket`.
If you expand the macro, you'll see that the generated code is functionally equivalent to the one you wrote manually,
although a bit more cumbersome to read:

```rust
#[automatically_derived]
impl ::core::cmp::PartialEq for Ticket {
    #[inline]
    fn eq(&self, other: &Ticket) -> bool {
        self.title == other.title && self.description == other.description
            && self.status == other.status
    }
}
```

The compiler will nudge you to derive traits when possible.

## Further reading

- [The little book of Rust macros](https://veykril.github.io/tlborm/)
- [Proc macro workshop](https://github.com/dtolnay/proc-macro-workshop)
