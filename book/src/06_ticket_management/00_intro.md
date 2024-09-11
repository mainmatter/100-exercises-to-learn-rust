# Intro

In the previous chapter we modelled `Ticket` in a vacuum: we defined its fields and their constraints, we learned
how to best represent them in Rust, but we didn't consider how `Ticket` fits into a larger system.
We'll use this chapter to build a simple workflow around `Ticket`, introducing a (rudimentary) management system to
store and retrieve tickets.

The task will give us an opportunity to explore new Rust concepts, such as:

- Stack-allocated arrays
- `Vec`, a growable array type
- `Iterator` and `IntoIterator`, for iterating over collections
- Slices (`&[T]`), to work with parts of a collection
- Lifetimes, to describe how long references are valid
- `HashMap` and `BTreeMap`, two key-value data structures
- `Eq` and `Hash`, to compare keys in a `HashMap`
- `Ord` and `PartialOrd`, to work with a `BTreeMap`
- `Index` and `IndexMut`, to access elements in a collection
