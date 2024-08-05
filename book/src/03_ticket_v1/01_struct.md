# Structs

We need to keep track of three pieces of information for each ticket:

- A title
- A description
- A status

We can start by using a [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
to represent them. `String` is the type defined in Rust's standard library to represent
[UTF-8 encoded](https://en.wikipedia.org/wiki/UTF-8) text.

But how do we **combine** these three pieces of information into a single entity?

## Defining a `struct`

A `struct` defines a **new Rust type**.

```rust
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

A struct is quite similar to what you would call a class or an object in other programming languages.

## Defining fields

The new type is built by combining other types as **fields**.\
Each field must have a name and a type, separated by a colon, `:`. If there are multiple fields, they are separated by a comma, `,`.

Fields don't have to be of the same type, as you can see in the `Configuration` struct below:

```rust
struct Configuration {
   version: u32,
   active: bool
}
```

## Instantiation

You can create an instance of a struct by specifying the values for each field:

```rust
// Syntax: <StructName> { <field_name>: <value>, ... }
let ticket = Ticket {
    title: "Build a ticket system".into(),
    description: "A Kanban board".into(),
    status: "Open".into()
};
```

## Accessing fields

You can access the fields of a struct using the `.` operator:

```rust
// Field access
let x = ticket.description;
```

## Methods

We can attach behaviour to our structs by defining **methods**.\
Using the `Ticket` struct as an example:

```rust
impl Ticket {
    fn is_open(self) -> bool {
        self.status == "Open"
    }
}

// Syntax:
// impl <StructName> {
//    fn <method_name>(<parameters>) -> <return_type> {
//        // Method body
//    }
// }
```

Methods are pretty similar to functions, with two key differences:

1. methods must be defined inside an **`impl` block**
2. methods may use `self` as their first parameter.
   `self` is a keyword and represents the instance of the struct the method is being called on.

### `self`

If a method takes `self` as its first parameter, it can be called using the **method call syntax**:

```rust
// Method call syntax: <instance>.<method_name>(<parameters>)
let is_open = ticket.is_open();
```

This is the same calling syntax you used to perform saturating arithmetic operations on `u32` values
in [the previous chapter](../02_basic_calculator/09_saturating.md).

### Static methods

If a method doesn't take `self` as its first parameter, it's a **static method**.

```rust
struct Configuration {
    version: u32,
    active: bool
}

impl Configuration {
    // `default` is a static method on `Configuration`
    fn default() -> Configuration {
        Configuration { version: 0, active: false }
    }
}
```

The only way to call a static method is by using the **function call syntax**:

```rust
// Function call syntax: <StructName>::<method_name>(<parameters>)
let default_config = Configuration::default();
```

### Equivalence

You can use the function call syntax even for methods that take `self` as their first parameter:

```rust
// Function call syntax:
//   <StructName>::<method_name>(<instance>, <parameters>)
let is_open = Ticket::is_open(ticket);
```

The function call syntax makes it quite clear that `ticket` is being used as `self`, the first parameter of the method,
but it's definitely more verbose. Prefer the method call syntax when possible.
