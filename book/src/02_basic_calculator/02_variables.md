# Variables

In Rust, you can use the `let` keyword to declare **variables**.\
For example:

```rust
let x = 42;
```

Above we defined a variable `x` and assigned it the value `42`.

## Type

Every variable in Rust must have a type. It can either be inferred by the compiler or explicitly specified by the
developer.

### Explicit type annotation

You can specify the variable type by adding a colon `:` followed by the type after the variable name. For example:

```rust
// let <variable_name>: <type> = <expression>;
let x: u32 = 42;
```

In the example above, we explicitly constrained the type of `x` to be `u32`.

### Type inference

If we don't specify the type of a variable, the compiler will try to infer it based on the context in which the variable
is used.

```rust
let x = 42;
let y: u32 = x;
```

In the example above, we didn't specify the type of `x`.\
`x` is later assigned to `y`, which is explicitly typed as `u32`. Since Rust doesn't perform automatic type coercion,
the compiler infers the type of `x` to be `u32`—the same as `y` and the only type that will allow the program to compile
without errors.

### Inference limitations

The compiler sometimes needs a little help to infer the correct variable type based on its usage.\
In those cases you'll get a compilation error and the compiler will ask you to provide an explicit type hint to
disambiguate the situation.

## Function arguments are variables

Not all heroes wear capes, not all variables are declared with `let`.\
Function arguments are variables too!

```rust
fn add_one(x: u32) -> u32 {
    x + 1
}
```

In the example above, `x` is a variable of type `u32`.\
The only difference between `x` and a variable declared with `let` is that functions arguments **must** have their type
explicitly declared. The compiler won't infer it for you.\
This constraint allows the Rust compiler (and us humans!) to understand the function's signature without having to look
at its implementation. That's a big boost for compilation speed[^speed]!

## Initialization

You don't have to initialize a variable when you declare it.\
For example

```rust
let x: u32;
```

is a valid variable declaration.\
However, you must initialize the variable before using it. The compiler will throw an error if you don't:

```rust
let x: u32;
let y = x + 1;
```

will throw a compilation error:

```text
error[E0381]: used binding `x` isn't initialized
 --> src/main.rs:3:9
  |
2 | let x: u32;
  |     - binding declared here but left uninitialized
3 | let y = x + 1;
  |         ^ `x` used here but it isn't initialized
  |
help: consider assigning a value
  |
2 | let x: u32 = 0;
  |            +++
```

[^speed]: The Rust compiler needs all the help it can get when it comes to compilation speed.
