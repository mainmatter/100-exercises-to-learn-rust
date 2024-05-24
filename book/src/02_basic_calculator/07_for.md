# Loops, part 2: `for`

Having to manually increment a counter variable is somewhat tedious. The pattern is also extremely common!\
To make this easier, Rust provides a more concise way to iterate over a range of values: the `for` loop.

## The `for` loop

A `for` loop is a way to execute a block of code for each element in an iterator[^iterator].

Here's the general syntax:

```rust
for <element> in <iterator> {
    // code to execute
}
```

## Ranges

Rust's standard library provides **range** type that can be used to iterate over a sequence of numbers[^weird-ranges].

For example, if we want to sum the numbers from 1 to 5:

```rust
let mut sum = 0;
for i in 1..=5 {
    sum += i;
}
```

Every time the loop runs, `i` will be assigned the next value in the range before executing the block of code.

There are five kinds of ranges in Rust:

- `1..5`: A (half-open) range. It includes all numbers from 1 to 4. It doesn't include the last value, 5.
- `1..=5`: An inclusive range. It includes all numbers from 1 to 5. It includes the last value, 5.
- `1..`: An open-ended range. It includes all numbers from 1 to infinity (well, until the maximum value of the integer type).
- `..5`: A range that starts at the minimum value for the integer type and ends at 4. It doesn't include the last value, 5.
- `..=5`: A range that starts at the minimum value for the integer type and ends at 5. It includes the last value, 5.

You can use a `for` loop with the first three kinds of ranges, where the starting point
is explicitly specified. The last two range types are used in other contexts, that we'll cover later.

The extreme values of a range don't have to be integer literals—they can be variables or expressions too!

For example:

```rust
let end = 5;
let mut sum = 0;

for i in 1..(end + 1) {
    sum += i;
}
```

## Further reading

- [`for` loop documentation](https://doc.rust-lang.org/std/keyword.for.html)

[^iterator]: Later in the course we'll give a precise definition of what counts as an "iterator".
For now, think of it as a sequence of values that you can loop over.
[^weird-ranges]: You can use ranges with other types too (e.g. characters and IP addresses),
but integers are definitely the most common case in day-to-day Rust programming.
