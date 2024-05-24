# `impl Trait` in argument position

In the previous section, we saw how `impl Trait` can be used to return a type without specifying its name.\
The same syntax can also be used in **argument position**:

```rust
fn print_iter(iter: impl Iterator<Item = i32>) {
    for i in iter {
        println!("{}", i);
    }
}
```

`print_iter` takes an iterator of `i32`s and prints each element.\
When used in **argument position**, `impl Trait` is equivalent to a generic parameter with a trait bound:

```rust
fn print_iter<T>(iter: T) 
where
    T: Iterator<Item = i32>
{
    for i in iter {
        println!("{}", i);
    }
}
```

## Downsides

As a rule of thumb, prefer generics over `impl Trait` in argument position.\
Generics allow the caller to explicitly specify the type of the argument, using the turbofish syntax (`::<>`),
which can be useful for disambiguation. That's not the case with `impl Trait`.
