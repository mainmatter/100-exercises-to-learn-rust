# Combinators

Iterators can do so much more than `for` loops!\
If you look at the documentation for the `Iterator` trait, you'll find a **vast** collection of
methods that you can leverage to transform, filter, and combine iterators in various ways.

Let's mention the most common ones:

- `map` applies a function to each element of the iterator.
- `filter` keeps only the elements that satisfy a predicate.
- `filter_map` combines `filter` and `map` in one step.
- `cloned` converts an iterator of references into an iterator of values, cloning each element.
- `enumerate` returns a new iterator that yields `(index, value)` pairs.
- `skip` skips the first `n` elements of the iterator.
- `take` stops the iterator after `n` elements.
- `chain` combines two iterators into one.

These methods are called **combinators**.\
They are usually **chained** together to create complex transformations in a concise and readable way:

```rust
let numbers = vec![1, 2, 3, 4, 5];
// The sum of the squares of the even numbers
let outcome: u32 = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    .sum();
```

## Closures

What's going on with the `filter` and `map` methods above?\
They take **closures** as arguments.

Closures are **anonymous functions**, i.e. functions that are not defined using the `fn` syntax we are used to.\
They are defined using the `|args| body` syntax, where `args` are the arguments and `body` is the function body.
`body` can be a block of code or a single expression.
For example:

```rust
// An anonymous function that adds 1 to its argument
let add_one = |x| x + 1;
// Could be written with a block too:
let add_one = |x| { x + 1 };
```

Closures can take more than one argument:

```rust
let add = |x, y| x + y;
let sum = add(1, 2);
```

They can also capture variables from their environment:

```rust
let x = 42;
let add_x = |y| x + y;
let sum = add_x(1);
```

If necessary, you can specify the types of the arguments and/or the return type:

```rust
// Just the input type
let add_one = |x: i32| x + 1;
// Or both input and output types, using the `fn` syntax
let add_one: fn(i32) -> i32 = |x| x + 1;
```

## `collect`

What happens when you're done transforming an iterator using combinators?\
You either iterate over the transformed values using a `for` loop, or you collect them into a collection.

The latter is done using the `collect` method.\
`collect` consumes the iterator and collects its elements into a collection of your choice.

For example, you can collect the squares of the even numbers into a `Vec`:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let squares_of_evens: Vec<u32> = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    .collect();
```

`collect` is generic over its **return type**.\
Therefore you usually need to provide a type hint to help the compiler infer the correct type.
In the example above, we annotated the type of `squares_of_evens` to be `Vec<u32>`.
Alternatively, you can use the **turbofish syntax** to specify the type:

```rust
let squares_of_evens = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    // Turbofish syntax: `<method_name>::<type>()`
    // It's called turbofish because `::<>` looks like a fish
    .collect::<Vec<u32>>();
```

## Further reading

- [`Iterator`'s documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html) gives you an
  overview of the methods available for iterators in `std`.
- [The `itertools` crate](https://docs.rs/itertools/) defines even **more** combinators for iterators.
