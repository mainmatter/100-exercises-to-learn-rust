# Mutable indexing

`Index` allows read-only access. It doesn't let you mutate the value you
retrieved.

## `IndexMut`

If you want to allow mutability, you need to implement the `IndexMut` trait.

```rust
// Slightly simplified
pub trait IndexMut<Idx>: Index<Idx>
{
    // Required method
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}
```

`IndexMut` can only be implemented if the type already implements `Index`,
since it unlocks an _additional_ capability.
