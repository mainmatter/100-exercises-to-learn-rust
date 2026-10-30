# Case-by-case behavior

`overflow-checks` is a blunt tool: it's a global setting that affects the whole program.\
It often happens that you want to handle integer overflows differently depending on the context: sometimes
wrapping is the right choice, other times panicking is preferable.

## `wrapping_` methods

You can opt into wrapping arithmetic on a per-operation basis by using the `wrapping_` methods[^method].\
For example, you can use `wrapping_add` to add two integers with wrapping:

```rust
let x = 255u8;
let y = 1u8;
let sum = x.wrapping_add(y);
assert_eq!(sum, 0);
```

## `saturating_` methods

Alternatively, you can opt into **saturating arithmetic** by using the `saturating_` methods.\
Instead of wrapping around, saturating arithmetic will return the maximum or minimum value for the integer type.
For example:

```rust
let x = 255u8;
let y = 1u8;
let sum = x.saturating_add(y);
assert_eq!(sum, 255);
```

Since `255 + 1` is `256`, which is bigger than `u8::MAX`, the result is `u8::MAX` (255).\
The opposite happens for underflows: `0 - 1` is `-1`, which is smaller than `u8::MIN`, so the result is `u8::MIN` (0).

You can't get saturating arithmetic via the `overflow-checks` profile setting—you have to explicitly opt into it
when performing the arithmetic operation.

A word of caution: unlike operators such as `+` and `*`, methods like `saturating_add` and `saturating_mul` are
**not** generic—each, it's implemented separately for every integer type. This can confuse Rust's type inference,
and the compiler may ask you to explicitly annotate the type it couldn't figure out on its own. As the very
first exercise reminded you, the compiler is your pair programming partner. Sometimes it just needs a bit of
help from you to move forward.

[^method]: You can think of methods as functions that are "attached" to a specific type.
We'll cover methods (and how to define them) in the next chapter.
