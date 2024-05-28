# Ownership

If you solved the previous exercise using what this course has taught you so far,
your accessor methods probably look like this:

```rust
impl Ticket {
    pub fn title(self) -> String {
        self.title
    }

    pub fn description(self) -> String {
        self.description
    }

    pub fn status(self) -> String {
        self.status
    }
}
```

Those methods compile and are enough to get tests to pass, but in a real-world scenario they won't get you very far.
Consider this snippet:

```rust
if ticket.status() == "To-Do" {
    // We haven't covered the `println!` macro yet,
    // but for now it's enough to know that it prints 
    // a (templated) message to the console
    println!("Your next task is: {}", ticket.title());
}
```

If you try to compile it, you'll get an error:

```text
error[E0382]: use of moved value: `ticket`
  --> src/main.rs:30:43
   |
25 |     let ticket = Ticket::new(/* */);
   |         ------ move occurs because `ticket` has type `Ticket`, 
   |                which does not implement the `Copy` trait
26 |     if ticket.status() == "To-Do" {
   |               -------- `ticket` moved due to this method call
...
30 |         println!("Your next task is: {}", ticket.title());
   |                                           ^^^^^^ value used here after move
   |
note: `Ticket::status` takes ownership of the receiver `self`, which moves `ticket`
  --> src/main.rs:12:23
   |
12 |         pub fn status(self) -> String {
   |                       ^^^^
```

Congrats, this is your first borrow-checker error!

## The perks of Rust's ownership system

Rust's ownership system is designed to ensure that:

- Data is never mutated while it's being read
- Data is never read while it's being mutated
- Data is never accessed after it has been destroyed

These constraints are enforced by the **borrow checker**, a subsystem of the Rust compiler,
often the subject of jokes and memes in the Rust community.

Ownership is a key concept in Rust, and it's what makes the language unique.
Ownership enables Rust to provide **memory safety without compromising performance**.
All these things are true at the same time for Rust:

1. There is no runtime garbage collector
2. As a developer, you rarely have to manage memory directly
3. You can't cause dangling pointers, double frees, and other memory-related bugs

Languages like Python, JavaScript, and Java give you 2. and 3., but not 1.\
Language like C or C++ give you 1., but neither 2. nor 3.

Depending on your background, 3. might sound a bit arcane: what is a "dangling pointer"?
What is a "double free"? Why are they dangerous?\
Don't worry: we'll cover these concepts in more details during the rest of the course.

For now, though, let's focus on learning how to work within Rust's ownership system.

## The owner

In Rust, each value has an **owner**, statically determined at compile-time.
There is only one owner for each value at any given time.

## Move semantics

Ownership can be transferred.

If you own a value, for example, you can transfer ownership to another variable:

```rust
let a = "hello, world".to_string(); // <--- `a` is the owner of the String
let b = a;  // <--- `b` is now the owner of the String
```

Rust's ownership system is baked into the type system: each function has to declare in its signature
_how_ it wants to interact with its arguments.

So far, all our methods and functions have **consumed** their arguments: they've taken ownership of them.
For example:

```rust
impl Ticket {
    pub fn description(self) -> String {
        self.description
    }
}
```

`Ticket::description` takes ownership of the `Ticket` instance it's called on.\
This is known as **move semantics**: ownership of the value (`self`) is **moved** from the caller to
the callee, and the caller can't use it anymore.

That's exactly the language used by the compiler in the error message we saw earlier:

```text
error[E0382]: use of moved value: `ticket`
  --> src/main.rs:30:43
   |
25 |     let ticket = Ticket::new(/* */);
   |         ------ move occurs because `ticket` has type `Ticket`, 
   |                which does not implement the `Copy` trait
26 |     if ticket.status() == "To-Do" {
   |               -------- `ticket` moved due to this method call
...
30 |         println!("Your next task is: {}", ticket.title());
   |                                           ^^^^^^ value used here after move
   |
note: `Ticket::status` takes ownership of the receiver `self`, which moves `ticket`
  --> src/main.rs:12:23
   |
12 |         pub fn status(self) -> String {
   |                       ^^^^
```

In particular, this is the sequence of events that unfold when we call `ticket.status()`:

- `Ticket::status` takes ownership of the `Ticket` instance
- `Ticket::status` extracts `status` from `self` and transfers ownership of `status` back to the caller
- The rest of the `Ticket` instance is discarded (`title` and `description`)

When we try to use `ticket` again via `ticket.title()`, the compiler complains: the `ticket` value is gone now,
we no longer own it, therefore we can't use it anymore.

To build _useful_ accessor methods we need to start working with **references**.

## Borrowing

It is desirable to have methods that can read the value of a variable without taking ownership of it.\
Programming would be quite limited otherwise. In Rust, that's done via **borrowing**.

Whenever you borrow a value, you get a **reference** to it.\
References are tagged with their privileges[^refine]:

- Immutable references (`&`) allow you to read the value, but not to mutate it
- Mutable references (`&mut`) allow you to read and mutate the value

Going back to the goals of Rust's ownership system:

- Data is never mutated while it's being read
- Data is never read while it's being mutated

To ensure these two properties, Rust has to introduce some restrictions on references:

- You can't have a mutable reference and an immutable reference to the same value at the same time
- You can't have more than one mutable reference to the same value at the same time
- The owner can't mutate the value while it's being borrowed
- You can have as many immutable references as you want, as long as there are no mutable references

In a way, you can think of an immutable reference as a "read-only" lock on the value,
while a mutable reference is like a "read-write" lock.

All these restrictions are enforced at compile-time by the borrow checker.

### Syntax

How do you borrow a value, in practice?\
By adding `&` or `&mut` **in front a variable**, you're borrowing its value.
Careful though! The same symbols (`&` and `&mut`) in **front of a type** have a different meaning:
they denote a different type, a reference to the original type.

For example:

```rust
struct Configuration {
    version: u32,
    active: bool,
}

fn main() {
    let config = Configuration {
        version: 1,
        active: true,
    };
    // `b` is a reference to the `version` field of `config`.
    // The type of `b` is `&u32`, since it contains a reference to a `u32` value.
    // We create a reference by borrowing `config.version`, using the `&` operator.
    // Same symbol (`&`), different meaning depending on the context!
    let b: &u32 = &config.version;
    //     ^ The type annotation is not necessary, 
    //       it's just there to clarify what's going on
}
```

The same concept applies to function arguments and return types:

```rust
// `f` takes a mutable reference to a `u32` as an argument, 
// bound to the name `number`
fn f(number: &mut u32) -> &u32 {
    // [...]
}
```

## Breathe in, breathe out

Rust's ownership system can be a bit overwhelming at first.\
But don't worry: it'll become second nature with practice.\
And you're going to get a lot of practice over the rest of this chapter, as well as the rest of the course!
We'll revisit each concept multiple times to make sure you get familiar with them
and truly understand how they work.

Towards the end of this chapter we'll explain _why_ Rust's ownership system is designed the way it is.
For the time being, focus on understanding the _how_. Take each compiler error as a learning opportunity!

[^refine]: This is a great mental model to start out, but it doesn't capture the _full_ picture.
We'll refine our understanding of references [later in the course](../07_threads/06_interior_mutability.md).
