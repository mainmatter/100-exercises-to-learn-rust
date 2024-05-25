# Validation

Let's go back to our ticket definition:

```rust
struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

We are using "raw" types for the fields of our `Ticket` struct.
This means that users can create a ticket with an empty title, a suuuuuuuper long description or
a nonsensical status (e.g. "Funny").\
We can do better than that!

## Further reading

- Check out [`String`'s documentation](https://doc.rust-lang.org/std/string/struct.String.html)
  for a thorough overview of the methods it provides. You'll need it for the exercise!
