# Mutable references

Your accessor methods should look like this now:

```rust
impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}
```

A sprinkle of `&` here and there did the trick!\
We now have a way to access the fields of a `Ticket` instance without consuming it in the process.
Let's see how we can enhance our `Ticket` struct with **setter methods** next.

## Setters

Setter methods allow users to change the values of `Ticket`'s private fields while making sure that its invariants
are respected (i.e. you can't set a `Ticket`'s title to an empty string).

There are two common ways to implement setters in Rust:

- Taking `self` as input.
- Taking `&mut self` as input.

### Taking `self` as input

The first approach looks like this:

```rust
impl Ticket {
    pub fn set_title(mut self, new_title: String) -> Self {
        // Validate the new title [...]
        self.title = new_title;
        self
    }
}
```

It takes ownership of `self`, changes the title, and returns the modified `Ticket` instance.\
This is how you'd use it:

```rust
let ticket = Ticket::new(
    "Title".into(), 
    "Description".into(), 
    "To-Do".into()
);
let ticket = ticket.set_title("New title".into());
```

Since `set_title` takes ownership of `self` (i.e. it **consumes it**), we need to reassign the result to a variable.
In the example above we take advantage of **variable shadowing** to reuse the same variable name: when
you declare a new variable with the same name as an existing one, the new variable **shadows** the old one. This
is a common pattern in Rust code.

`self`-setters work quite nicely when you need to change multiple fields at once: you can chain multiple calls together!

```rust
let ticket = ticket
    .set_title("New title".into())
    .set_description("New description".into())
    .set_status("In Progress".into());
```

### Taking `&mut self` as input

The second approach to setters, using `&mut self`, looks like this instead:

```rust
impl Ticket {
    pub fn set_title(&mut self, new_title: String) {
        // Validate the new title [...]
        
        self.title = new_title;
    }
}
```

This time the method takes a mutable reference to `self` as input, changes the title, and that's it.
Nothing is returned.

You'd use it like this:

```rust
let mut ticket = Ticket::new(
    "Title".into(),
    "Description".into(),
    "To-Do".into()
);
ticket.set_title("New title".into());

// Use the modified ticket
```

Ownership stays with the caller, so the original `ticket` variable is still valid. We don't need to reassign the result.
We need to mark `ticket` as mutable though, because we're taking a mutable reference to it.

`&mut`-setters have a downside: you can't chain multiple calls together.
Since they don't return the modified `Ticket` instance, you can't call another setter on the result of the first one.
You have to call each setter separately:

```rust
ticket.set_title("New title".into());
ticket.set_description("New description".into());
ticket.set_status("In Progress".into());
```
