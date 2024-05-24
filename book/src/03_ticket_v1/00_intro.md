# Modelling A Ticket

The first chapter should have given you a good grasp over some of Rust's primitive types, operators and
basic control flow constructs.\
In this chapter we'll go one step further and cover what makes Rust truly unique: **ownership**.\
Ownership is what enables Rust to be both memory-safe and performant, with no garbage collector.

As our running example, we'll use a (JIRA-like) ticket, the kind you'd use to track bugs, features, or tasks in
a software project.\
We'll take a stab at modeling it in Rust. It'll be the first iteration—it won't be perfect nor very idiomatic
by the end of the chapter. It'll be enough of a challenge though!\
To move forward you'll have to pick up several new Rust concepts, such as:

- `struct`s, one of Rust's ways to define custom types
- Ownership, references and borrowing
- Memory management: stack, heap, pointers, data layout, destructors
- Modules and visibility
- Strings
