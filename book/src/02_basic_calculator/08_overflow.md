# Overflow

The factorial of a number grows quite fast.  
For example, the factorial of 20 is 2,432,902,008,176,640,000. That's already bigger than the maximum value for a
32-bit integer, 2,147,483,647.

When the result of an arithmetic operation is bigger than the maximum value for a given integer type,
we are talking about **an integer overflow**.

Integer overflows are an issue because they violate the contract for arithmetic operations.  
The result of an arithmetic operation between two integers of a given type should be another integer of the same type.
But the _mathematically correct result_ doesn't fit into that integer type!

> If the result is smaller than the minimum value for a given integer type, we refer to the event as **an integer
> underflow**.  
> For brevity, we'll only talk about integer overflows for the rest of this section, but keep in mind that
> everything we say applies to integer underflows as well.
>
> The `speed` function you wrote in the ["Variables" section](02_variables.md) underflowed for some input
> combinations.
> E.g. if `end` is smaller than `start`, `end - start` will underflow the `u32` type since the result is supposed
> to be negative but `u32` can't represent negative numbers.

## No automatic promotion

One possible approach would be automatically promote the result to a bigger integer type.
E.g. if you're summing two `u8` integers and the result is 256 (`u8::MAX + 1`), Rust could choose to interpret the
result as `u16`, the next integer type that's big enough to hold 256.

But, as we've discussed before, Rust is quite picky about type conversions. Automatic integer promotion
is not Rust's solution to the integer overflow problem.

## Alternatives

Since we ruled out automatic promotion, what can we do when an integer overflow occurs?  
It boils down to two different approaches:

- Reject the operation
- Come up with a "sensible" result that fits into the expected integer type

### Reject the operation

This is the most conservative approach: we stop the program when an integer overflow occurs.  
That's done via a panic, the mechanism we've already seen in the ["Panics" section](04_panics.md).

### Come up with a "sensible" result

When the result of an arithmetic operation is bigger than the maximum value for a given integer type, you can
choose to **wrap around**.  
If you think of all the possible values for a given integer type as a circle, wrapping around means that when you
reach the maximum value, you start again from the minimum value.

For example, if you do a **wrapping addition** between 1 and 255 (=`u8::MAX`), the result is 0 (=`u8::MIN`).
If you're working with signed integers, the same principle applies. E.g. adding 1 to 127 (=`i8::MAX`) with wrapping
will give you -128 (=`i8::MIN`).

## `overflow-checks`

Rust lets you, the developer, choose which approach to use when an integer overflow occurs.
The behaviour is controlled by the `overflow-checks` profile setting.

If `overflow-checks` is set to `true`, Rust will **panic at runtime** when an integer operation overflows.
If `overflow-checks` is set to `false`, Rust will **wrap around** when an integer operation overflows.

You may be wondering—what is a profile setting? Let's get into that!

## Profiles

A [**profile**](https://doc.rust-lang.org/cargo/reference/profiles.html) is a set of configuration options that can be
used to customize the way Rust code is compiled.

Cargo provides two built-in profiles: `dev` and `release`.  
The `dev` profile is used every time you run `cargo build`, `cargo run` or `cargo test`. It's aimed at local
development,
therefore it sacrifices runtime performance in favor of faster compilation times and a better debugging experience.  
The `release` profile, instead, is optimized for runtime performance but incurs longer compilation times. You need
to explicitly request via the `--release` flag—e.g. `cargo build --release` or `cargo run --release`.

> "Have you built your project in release mode?" is almost a meme in the Rust community.  
> It refers to developers who are not familiar with Rust and complain about its performance on
> social media (e.g. Reddit, Twitter, etc.) before realizing they haven't built their project in
> release mode.

You can also define custom profiles or customize the built-in ones.

### `overflow-check`

By default, `overflow-checks` is set to:

- `true` for the `dev` profile
- `false` for the `release` profile

This is in line with the goals of the two profiles.  
`dev` is aimed at local development, so it panics in order to highlight potential issues as early as possible.  
`release`, instead, is tuned for runtime performance: checking for overflows would slow down the program, so it
prefers to wrap around.

At the same time, having different behaviours for the two profiles can lead to subtle bugs.  
Our recommendation is to enable `overflow-checks` for both profiles: it's better to crash than to silently produce
incorrect results. The runtime performance hit is negligible in most cases; if you're working on a performance-critical
application, you can run benchmarks to decide if it's something you can afford.

## References

- The exercise for this section is located in `exercises/02_basic_calculator/08_overflow`

## Further reading

- Check out ["Myths and legends about integer overflow in Rust"](https://huonw.github.io/blog/2016/04/myths-and-legends-about-integer-overflow-in-rust/)
for an in-depth discussion about integer overflow in Rust.
