# The `Drop` trait

When we introduced [destructors](../03_ticket_v1/11_destructor),
we mentioned that the `drop` function:

1. reclaims the memory occupied by the type (i.e. `std::mem::size_of` bytes)
2. cleans up any additional resources that the value might be managing (e.g. the heap buffer of a `String`)

Step 2. is where the `Drop` trait comes in.

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

The `Drop` trait is a mechanism for you to define _additional_ cleanup logic for your types,
beyond what the compiler does for you automatically.  
Whatever you put in the `drop` method will be executed when the value goes out of scope.

## `Drop` and `Copy`

When talking about the `Copy` trait, we said that a type can't implement `Copy` if it
manages additional resources beyond the `std::mem::size_of` bytes that it occupies in memory.

You might wonder: how does the compiler know if a type manages additional resources?
That's right: `Drop` trait implementations!  
If your type has an explicit `Drop` implementation, the compiler will assume
that your type has additional resources attached to it and won't allow you to implement `Copy`.

```rust
// This is a unit struct, i.e. a struct with no fields.
#[derive(Clone, Copy)]
struct MyType;

impl Drop for MyType {
    fn drop(&mut self) {
       // We don't need to do anything here,
       // it's enough to have an "empty" Drop implementation
    }
}
```

The compiler will complain with this error message:

```text
error[E0184]: the trait `Copy` cannot be implemented for this type; the type has a destructor
 --> src/lib.rs:2:17
  |
2 | #[derive(Clone, Copy)]
  |                 ^^^^ `Copy` not allowed on types with destructors
```

## References

- The exercise for this section is located in `exercises/04_traits/13_drop`
