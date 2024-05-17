# Syntax

<div class="warning">

Don't jump ahead!  
Complete the exercise for the previous section before you start this one.  
It's located in `exercises/01_intro/00_welcome`, in the [course GitHub's repository](https://github.com/mainmatter/100-exercises-to-learn-rust).  
Use [`wr`](00_welcome.md#wr-the-workshop-runner) to start the course and verify your solutions.

</div>

The previous task doesn't even qualify as an exercise, but it already exposed you to quite a bit of Rust **syntax**.
We won't cover every single detail of Rust's syntax used in the previous exercise.
Instead, we'll cover _just enough_ to keep going without getting stuck in the details.  
One step at a time!

## Comments

You can use `//` for single-line comments:

```rust
// This is a single-line comment
// Followed by another single-line comment
```

## Functions

Functions in Rust are defined using the `fn` keyword, followed by the function's name, its input parameters, and its
return type.
The function's body is enclosed in curly braces `{}`.

In previous exercise, you saw the `greeting` function:

```rust
// `fn` <function_name> ( <input parameters> ) -> <return_type> { <body> }
fn greeting() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to __!"
}
```

`greeting` has no input parameters and returns a reference to a string slice (`&'static str`).

### Return type

The return type can be omitted from the signature if the function doesn't return anything (i.e. if it returns `()`,
Rust's unit type).
That's what happened with the `test_welcome` function:

```rust
fn test_welcome() {
    assert_eq!(greeting(), "I'm ready to learn Rust!");
}
```

The above is equivalent to:

```rust
// Spelling out the unit return type explicitly
//                   👇
fn test_welcome() -> () {
    assert_eq!(greeting(), "I'm ready to learn Rust!");
}
```

### Returning values

The last expression in a function is implicitly returned:

```rust
fn greeting() -> &'static str {
    // This is the last expression in the function
    // Therefore its value is returned by `greeting`
    "I'm ready to learn Rust!"
}
```

You can also use the `return` keyword to return a value early:

```rust
fn greeting() -> &'static str {
    // Notice the semicolon at the end of the line!
    return "I'm ready to learn Rust!";
}
```

It is considered idiomatic to omit the `return` keyword when possible.

### Input parameters

Input parameters are declared inside the parentheses `()` that follow the function's name.  
Each parameter is declared with its name, followed by a colon `:`, followed by its type.

For example, the `greet` function below takes a `name` parameter of type `&str` (a "string slice"):

```rust
// An input parameter
//        👇
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

If there are multiple input parameters, they must be separated with commas.

### Type annotations

Since we've been mentioned "types" a few times, let's state it clearly: Rust is a **statically typed language**.  
Every single value in Rust has a type and that type must be known to the compiler at compile-time.

Types are a form of **static analysis**.  
You can think of a type as a **tag** that the compiler attaches to every value in your program. Depending on the
tag, the compiler can enforce different rules—e.g. you can't add a string to a number, but you can add two numbers
together.
If leveraged correctly, types can prevent whole classes of runtime bugs.

## References

- The exercise for this section is located in `exercises/01_intro/01_syntax`
