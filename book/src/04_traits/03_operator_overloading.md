# Operator overloading

Now that we have a basic understanding of what traits are, let's circle back to **operator overloading**.
Operator overloading is the ability to define custom behavior for operators like `+`, `-`, `*`, `/`, `==`, `!=`, etc.

## Operators are traits

In Rust, operators are traits.  
For each operator, there is a corresponding trait that defines the behavior of that operator.
By implementing that trait for your type, you **unlock** the usage of the corresponding operators.  

For example, the [`PartialEq` trait](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) defines the behavior of 
the `==` and `!=` operators:

```rust
// The `PartialEq` trait definition, from Rust's standard library
// (It is *slightly* simplified, for now)
pub trait PartialEq {
    // Required method
    //
    // `Self` is a Rust keyword that stands for 
    // "the type that is implementing the trait"
    fn eq(&self, other: &Self) -> bool;

    // Provided method
    fn ne(&self, other: &Self) -> bool { ... }
}
```

When you write `x == y` the compiler will look for an implementation of the `PartialEq` trait for the types of `x` and `y`
and replace `x == y` with `x.eq(y)`. It's syntactic sugar!

This is the correspondence for the main operators:

| Operator                 | Trait                                                                   |
|--------------------------|-------------------------------------------------------------------------|
| `+`                      | [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html)               |
| `-`                      | [`Sub`](https://doc.rust-lang.org/std/ops/trait.Sub.html)               |                                                 
| `*`                      | [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html)               |
| `/`                      | [`Div`](https://doc.rust-lang.org/std/ops/trait.Div.html)               |
| `%`                      | [`Rem`](https://doc.rust-lang.org/std/ops/trait.Rem.html)               |
| `==` and `!=`            | [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)   |
| `<`, `>`, `<=`, and `>=` | [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) |

Arithmetic operators live in the [`std::ops`](https://doc.rust-lang.org/std/ops/index.html) module,
while comparison ones live in the [`std::cmp`](https://doc.rust-lang.org/std/cmp/index.html) module.

## Default implementations

The comment on `PartialEq::ne` states that "`ne` is a provided method".  
It means that `PartialEq` provides a **default implementation** for `ne` in the trait definition—the `{ ... }` elided 
block in the definition snippet.  
If we expand the elided block, it looks like this:

```rust
pub trait PartialEq {
    fn eq(&self, other: &Self) -> bool;

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
```

It's what you expect: `ne` is the negation of `eq`.  
Since a default implementation is provided, you can skip implementing `ne` when you implement `PartialEq` for your type.
It's enough to implement `eq`:

```rust
struct WrappingU8 {
    inner: u8,
}

impl PartialEq for WrappingU8 {
    fn eq(&self, other: &WrappingU8) -> bool {
        self.inner == other.inner
    }
    
    // No `ne` implementation here
}
```

You are not forced to use the default implementation though. 
You can choose to override it when you implement the trait:

```rust
struct MyType;

impl PartialEq for MyType {
    fn eq(&self, other: &MyType) -> bool {
        // Custom implementation
    }

    fn ne(&self, other: &MyType) -> bool {
        // Custom implementation
    }
}
```

## References

- The exercise for this section is located in `exercises/04_traits/03_operator_overloading`
