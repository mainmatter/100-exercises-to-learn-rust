# `From` and `Into`

Let's go back to where our string journey started:

```rust
let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
```

We can now know enough to start unpacking what `.into()` is doing here.

## The problem

This is the signature of the `new` method:

```rust
impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Self {
        // [...]
    }
}
```

We've also seen that string literals (such as `"A title"`) are of type `&str`.  
We have a type mismatch here: a `String` is expected, but we have a `&str`. 
No magical coercion will come to save us this time; we need **to perform a conversion**.

## `From` and `Into`

The Rust standard library defines two traits for **infallible conversions**: `From` and `Into`, 
in the `std::convert` module.

```rust
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    fn into(self) -> T;
}
```

These trait definitions showcase a few concepts that we haven't seen before: **supertraits**, **generics**, 
and **implicit trait bounds**. Let's unpack those first.

### Supertrait / Subtrait

The `From: Sized` syntax implies that `From` is a **subtrait** of `Sized`: any type that
implements `From` must also implement `Sized`.
Alternatively, you could say that `Sized` is a **supertrait** of `From`.

### Generics

Both `From` and `Into` are **generic traits**.  
They take a type parameter, `T`, to refer to the type being converted from or into.
`T` is a placeholder for the actual type, which will be specified when the trait is implemented or used.

### Implicit trait bounds

Every time you have a generic type parameter, the compiler implicitly assumes that it's `Sized`.

For example:

```rust
pub struct Foo<T> {
    inner: T,
}
```

is actually equivalent to:

```rust
pub struct Foo<T> 
where
    T: Sized,
//  ^^^^^^^^^
//  This is known as a **trait bound**
//  It specifies that this implementation applies exclusively
//  to types `T` that implement `Sized`
//  You can require multiple trait to be implemented using 
//  the `+` sign. E.g. `Sized + PartialEq<T>`
{
    inner: T,
}
```

You can opt out of this behavior by using a **negative trait bound**:

```rust
// You can also choose to inline trait bounds,
// rather than using `where` clauses

pub struct Foo<T: ?Sized> {
    //            ^^^^^^^
    //            This is a negative trait bound
    inner: T,
}
```

This syntax reads as "`T` may or may not be `Sized`", and it allows you to
bind `T` to a DST (e.g. `Foo<str>`).  
In the case of `From<T>`, we want _both_ `T` and the type implementing `From<T>` to be `Sized`, even
though the former bound is implicit.

## `&str` to `String`

In [`std`'s documentation](https://doc.rust-lang.org/std/convert/trait.From.html#implementors) 
you can see which `std` types implement the `From` trait.  
You'll find that `String` implements `From<&str> for String`. Thus, we can write:

```rust
let title = String::from("A title");
```

We've been primarily using `.into()`, though.  
If you check out the [implementors of `Into`](https://doc.rust-lang.org/std/convert/trait.Into.html#implementors)
you won't find `Into<&str> for String`. What's going on?

`From` and `Into` are **dual traits**.  
In particular, `Into` is implemented for any type that implements `From` using a **blanket implementation**:

```rust
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

If a type `U` implements `From<T>`, then `Into<U> for T` is automatically implemented. That's why
we can write `let title = "A title".into();`.

## `.into()`

Every time you see `.into()`, you're witnessing a conversion between types.  
What's the target type, though?

In most cases, the target type is either:

- Specified by the signature of a function/method (e.g. `Ticket::new` in our example above)
- Specified in the variable declaration with a type annotation (e.g. `let title: String = "A title".into();`)

`.into()` will work out of the box as long as the compiler can infer the target type from the context without ambiguity.

## References

- The exercise for this section is located in `exercises/04_traits/08_from`
