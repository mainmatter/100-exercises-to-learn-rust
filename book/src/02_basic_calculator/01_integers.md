# Types, part 1

In the ["Syntax" section](../01_intro/01_syntax.md) `compute`'s input parameters were of type `u32`.\
Let's unpack what that _means_.

## Primitive types

`u32` is one of Rust's **primitive types**. Primitive types are the most basic building blocks of a language.
They're built into the language itself—i.e. they are not defined in terms of other types.

You can combine these primitive types to create more complex types. We'll see how soon enough.

## Integers

`u32`, in particular, is an **unsigned 32-bit integer**.

An integer is a number that can be written without a fractional component. E.g. `1` is an integer, while `1.2` is not.

### Signed vs. unsigned

An integer can be **signed** or **unsigned**.\
An unsigned integer can only represent non-negative numbers (i.e. `0` or greater).
A signed integer can represent both positive and negative numbers (e.g. `-1`, `12`, etc.).

The `u` in `u32` stands for **unsigned**.\
The equivalent type for signed integer is `i32`, where the `i` stands for integer (i.e. any integer, positive or
negative).

### Bit width

The `32` in `u32` refers to the **number of bits[^bit]** used to represent the number in memory.\
The more bits, the larger the range of numbers that can be represented.

Rust supports multiple bit widths for integers: `8`, `16`, `32`, `64`, `128`.

With 32 bits, `u32` can represent numbers from `0` to `2^32 - 1` (a.k.a. [`u32::MAX`](https://doc.rust-lang.org/std/primitive.u32.html#associatedconstant.MAX)).\
With the same number of bits, a signed integer (`i32`) can represent numbers from `-2^31` to `2^31 - 1`
(i.e. from [`i32::MIN`](https://doc.rust-lang.org/std/primitive.i32.html#associatedconstant.MIN)
to [`i32::MAX`](https://doc.rust-lang.org/std/primitive.i32.html#associatedconstant.MAX)).\
The maximum value for `i32` is smaller than the maximum value for `u32` because one bit is used to represent
the sign of the number. Check out the [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement)
representation for more details on how signed integers are represented in memory.

### Summary

Combining the two variables (signed/unsigned and bit width), we get the following integer types:

| Bit width | Signed | Unsigned |
| --------- | ------ | -------- |
| 8-bit     | `i8`   | `u8`     |
| 16-bit    | `i16`  | `u16`    |
| 32-bit    | `i32`  | `u32`    |
| 64-bit    | `i64`  | `u64`    |
| 128-bit   | `i128` | `u128`   |

## Literals

A **literal** is a notation for representing a fixed value in source code.\
For example, `42` is a Rust literal for the number forty-two.

### Type annotations for literals

But all values in Rust have a type, so... what's the type of `42`?

The Rust compiler will try to infer the type of a literal based on how it's used.\
If you don't provide any context, the compiler will default to `i32` for integer literals.\
If you want to use a different type, you can add the desired integer type as a suffix—e.g. `2u64` is a 2 that's
explicitly typed as a `u64`.

### Underscores in literals

You can use underscores `_` to improve the readability of large numbers.\
For example, `1_000_000` is the same as `1000000`.

## Arithmetic operators

Rust supports the following arithmetic operators[^traits] for integers:

- `+` for addition
- `-` for subtraction
- `*` for multiplication
- `/` for division
- `%` for remainder

Precedence and associativity rules for these operators are the same as in mathematics.\
You can use parentheses to override the default precedence. E.g. `2 * (3 + 4)`.

> ⚠️ **Warning**
>
> The division operator `/` performs integer division when used with integer types.
> I.e. the result is truncated towards zero. For example, `5 / 2` is `2`, not `2.5`.

## No automatic type coercion

As we discussed in the previous exercise, Rust is a statically typed language.\
In particular, Rust is quite strict about type coercion. It won't automatically convert a value from one type to
another[^coercion],
even if the conversion is lossless. You have to do it explicitly.

For example, you can't assign a `u8` value to a variable with type `u32`, even though all `u8` values are valid `u32`
values:

```rust
let b: u8 = 100;
let a: u32 = b;
```

It'll throw a compilation error:

```text
error[E0308]: mismatched types
  |
3 |     let a: u32 = b;
  |            ---   ^ expected `u32`, found `u8`
  |            |
  |            expected due to this
  |
```

We'll see how to convert between types [later in this course](../04_traits/09_from.md).

## Further reading

- [The integer types section](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types) in the official Rust book

[^bit]: A bit is the smallest unit of data in a computer. It can only have two values: `0` or `1`.

[^traits]: Rust doesn't let you define custom operators, but it puts you in control of how the built-in operators
behave.
We'll talk about operator overloading [later in the course](../04_traits/03_operator_overloading.md), after we've covered traits.

[^coercion]: There are some exceptions to this rule, mostly related to references, smart pointers and ergonomics. We'll
cover those [later on](../04_traits/07_deref.md).
A mental model of "all conversions are explicit" will serve you well in the meantime.
