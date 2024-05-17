# Conversions, pt. 1

We've repeated over and over again that Rust won't perform
implicit type conversions for integers.  
How do you perform _explicit_ conversions then?

## `as`

You can use the `as` operator to convert between integer types.  
`as` conversions are **infallible**.

For example:

```rust
let a: u32 = 10;

// Cast `a` into the `u64` type
let b = a as u64;

// You can use `_` as the target type
// if it can be correctly inferred 
// by the compiler. For example:
let c: u64 = a as _;
```

The semantics of this conversion are what you expect: all `u32` values are valid `u64`
values.  

### Truncation

Things get more interesting if we go in the opposite direction:

```rust
// A number that's too big 
// to fit into a `u8`
let a: u16 = 255 + 1;
let b = a as u8;
```

This program will run without issues, because `as` conversions are infallible.
But what is the value of `b`? 
When going from a larger integer type to a smaller, the Rust compiler will perform
a **truncation**. 

To understand what happens, let's start by looking at how `256u16` is 
represented in memory, as a sequence of bits:

```text
 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0
|               |               |
+---------------+---------------+
  First 8 bits    Last 8 bits
```

When converting to a `u8`, the Rust compiler will keep the last 8 bits of a `u16`
memory representation:

```text
 0 0 0 0 0 0 0 0 
|               |
+---------------+
  Last 8 bits   
```

Hence `256 as u8` is equal to `0`. That's... not ideal, in most scenarios.  
In fact, the Rust compiler will actively try to stop you if it sees you trying
to cast a literal value which will result in a truncation:

```text
error: literal out of range for `i8`
  |
4 |     let a = 255 as i8;
  |             ^^^
  |
  = note: the literal `255` does not fit into the type `i8` whose range is `-128..=127`
  = help: consider using the type `u8` instead
  = note: `#[deny(overflowing_literals)]` on by default
```

### Recommendation

As a rule of thumb, be quite careful with `as` casting.  
Use it _exclusively_ for going from a smaller type to a larger type. 
To convert from a larger to smaller integer type, rely on the 
[*fallible* conversion machinery](../05_ticket_v2/13_try_from) that we'll
explore later in the course.

### Limitations

Surprising behaviour is not the only downside of `as` casting. 
It is also fairly limited: you can only rely on `as` casting
for primitive types and a few other special cases.  
When working with composite types, you'll have to rely on 
different conversion mechanisms ([fallible](../05_ticket_v2/13_try_from) 
and [infallible](../04_traits/08_from)), which we'll explore later on.

## References

- The exercise for this section is located in `exercises/02_basic_calculator/10_as_casting`

## Further reading

- Check out [Rust's official reference](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast) 
  to learn the precise behaviour of `as` casting for each source/target combination, 
  as well as the exhaustive list of allowed conversions. 
