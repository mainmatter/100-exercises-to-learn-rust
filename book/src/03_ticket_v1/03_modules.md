# Modules

The `new` method you've just defined is trying to enforce some **constraints** on the field values for `Ticket`.
But are those invariants really enforced? What prevents a developer from creating a `Ticket`
without going through `Ticket::new`?

To get proper **encapsulation** you need to become familiar with two new concepts: **visibility** and **modules**.
Let's start with modules.

## What is a module?

In Rust a **module** is a way to group related code together, under a common namespace (i.e. the module's name).\
You've already seen modules in action: the unit tests that verify the correctness of your code are defined in a
different module, named `tests`.

```rust
#[cfg(test)]
mod tests {
    // [...]
}
```

## Inline modules

The `tests` module above is an example of an **inline module**: the module declaration (`mod tests`) and the module
contents (the stuff inside `{ ... }`) are next to each other.

## Module tree

Modules can be nested, forming a **tree** structure.\
The root of the tree is the **crate** itself, which is the top-level module that contains all the other modules.
For a library crate, the root module is usually `src/lib.rs` (unless its location has been customized).
The root module is also known as the **crate root**.

The crate root can have submodules, which in turn can have their own submodules, and so on.

## External modules and the filesystem

Inline modules are useful for small pieces of code, but as your project grows you'll want to split your code into
multiple files. In the parent module, you declare the existence of a submodule using the `mod` keyword.

```rust
mod dog;
```

`cargo`, Rust's build tool, is then in charge of finding the file that contains
the module implementation.\
If your module is declared in the root of your crate (e.g. `src/lib.rs` or `src/main.rs`),
`cargo` expects the file to be named either:

- `src/<module_name>.rs`
- `src/<module_name>/mod.rs`

If your module is a submodule of another module, the file should be named:

- `[..]/<parent_module>/<module_name>.rs`
- `[..]/<parent_module>/<module_name>/mod.rs`

E.g. `src/animals/dog.rs` or `src/animals/dog/mod.rs` if `dog` is a submodule of `animals`.

Your IDE might help you create these files automatically when you declare a new module using the `mod` keyword.

## Item paths and `use` statements

You can access items defined in the same module without any special syntax. You just use their name.

```rust
struct Ticket {
    // [...]
}

// No need to qualify `Ticket` in any way here
// because we're in the same module
fn mark_ticket_as_done(ticket: Ticket) {
    // [...]
}
```

That's not the case if you want to access an entity from a different module.\
You have to use a **path** pointing to the entity you want to access.

You can compose the path in various ways:

- starting from the root of the current crate, e.g. `crate::module_1::MyStruct`
- starting from the parent module, e.g. `super::my_function`
- starting from the current module, e.g. `sub_module_1::MyStruct`

Both `crate` and `super` are **keywords**.\
`crate` refers to the root of the current crate, while `super` refers to the parent of the current module.

Having to write the full path every time you want to refer to a type can be cumbersome.
To make your life easier, you can introduce a `use` statement to bring the entity into scope.

```rust
// Bring `MyStruct` into scope
use crate::module_1::module_2::MyStruct;

// Now you can refer to `MyStruct` directly
fn a_function(s: MyStruct) {
     // [...]
}
```

### Star imports

You can also import all the items from a module with a single `use` statement.

```rust
use crate::module_1::module_2::*;
```

This is known as a **star import**.\
It is generally discouraged because it can pollute the current namespace, making it hard to understand
where each name comes from and potentially introducing name conflicts.\
Nonetheless, it can be useful in some cases, like when writing unit tests. You might have noticed
that most of our test modules start with a `use super::*;` statement to bring all the items from the parent module
(the one being tested) into scope.

## Visualizing the module tree

If you're struggling to picture the module tree of your project, you can try using
[`cargo-modules`](https://crates.io/crates/cargo-modules) to visualize it!

Refer to their documentation for installation instructions and usage examples.
