# Generics and associated types

Let's re-examine the definition for two of the traits we studied so far, `From` and `Deref`:

```rust
pub trait From<T> {
    fn from(value: T) -> Self;
}

pub trait Deref {
    type Target;
    
    fn deref(&self) -> &Self::Target;
}
```

They both feature type parameters.\
In the case of `From`, it's a generic parameter, `T`.\
In the case of `Deref`, it's an associated type, `Target`.

What's the difference? Why use one over the other?

## At most one implementation

Due to how deref coercion works, there can only be one "target" type for a given type. E.g. `String` can
only deref to `str`.
It's about avoiding ambiguity: if you could implement `Deref` multiple times for a type,
which `Target` type should the compiler choose when you call a `&self` method?

That's why `Deref` uses an associated type, `Target`.\
An associated type is uniquely determined **by the trait implementation**.
Since you can't implement `Deref` more than once, you'll only be able to specify one `Target` for a given type
and there won't be any ambiguity.

## Generic traits

On the other hand, you can implement `From` multiple times for a type, **as long as the input type `T` is different**.
For example, you can implement `From` for `WrappingU32` using both `u32` and `u16` as input types:

```rust
impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { inner: value }
    }
}

impl From<u16> for WrappingU32 {
    fn from(value: u16) -> Self {
        WrappingU32 { inner: value.into() }
    }
}
```

This works because `From<u16>` and `From<u32>` are considered **different traits**.\
There is no ambiguity: the compiler can determine which implementation to use based on type of the value being converted.

## Case study: `Add`

As a closing example, consider the `Add` trait from the standard library:

```rust
pub trait Add<RHS = Self> {
    type Output;
    
    fn add(self, rhs: RHS) -> Self::Output;
}
```

It uses both mechanisms:

- it has a generic parameter, `RHS` (right-hand side), which defaults to `Self`
- it has an associated type, `Output`, the type of the result of the addition

### `RHS`

`RHS` is a generic parameter to allow for different types to be added together.\
For example, you'll find these two implementations in the standard library:

```rust
impl Add<u32> for u32 {
    type Output = u32;
    
    fn add(self, rhs: u32) -> u32 {
      //                      ^^^
      // This could be written as `Self::Output` instead.
      // The compiler doesn't care, as long as the type you
      // specify here matches the type you assigned to `Output` 
      // right above.
      // [...]
    }
}

impl Add<&u32> for u32 {
    type Output = u32;
    
    fn add(self, rhs: &u32) -> u32 {
        // [...]
    }
}
```

This allows the following code to compile:

```rust
let x = 5u32 + &5u32 + 6u32;
```

because `u32` implements `Add<&u32>` _as well as_ `Add<u32>`.

### `Output`

`Output` represents the type of the result of the addition.

Why do we need `Output` in the first place? Can't we just use `Self` as output, the type implementing `Add`?
We could, but it would limit the flexibility of the trait. In the standard library, for example, you'll find
this implementation:

```rust
impl Add<&u32> for &u32 {
    type Output = u32;

    fn add(self, rhs: &u32) -> u32 {
        // [...]
    }
}
```

The type they're implementing the trait for is `&u32`, but the result of the addition is `u32`.\
It would be impossible[^flexible] to provide this implementation if `add` had to return `Self`, i.e. `&u32` in this case.
`Output` lets `std` decouple the implementor from the return type, thus supporting this case.

On the other hand, `Output` can't be a generic parameter. The output type of the operation **must** be uniquely determined
once the types of the operands are known. That's why it's an associated type: for a given combination of implementor
and generic parameters, there is only one `Output` type.

## Conclusion

To recap:

- Use an **associated type** when the type must be uniquely determined for a given trait implementation.
- Use a **generic parameter** when you want to allow multiple implementations of the trait for the same type,
  with different input types.

[^flexible]: Flexibility is rarely free: the trait definition is more complex due to `Output`, and implementors have to reason about
what they want to return. The trade-off is only justified if that flexibility is actually needed. Keep that in mind
when designing your own traits.
