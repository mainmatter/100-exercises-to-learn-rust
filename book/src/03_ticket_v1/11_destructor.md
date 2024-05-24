# Destructors

When introducing the heap, we mentioned that you're responsible for freeing the memory you allocate.\
When introducing the borrow-checker, we also stated that you rarely have to manage memory directly in Rust.

These two statements might seem contradictory at first.
Let's see how they fit together by introducing **scopes** and **destructors**.

## Scopes

The **scope** of a variable is the region of Rust code where that variable is valid, or **alive**.

The scope of a variable starts with its declaration.
It ends when one of the following happens:

1. the block (i.e. the code between `{}`) where the variable was declared ends
   ```rust
   fn main() {
      // `x` is not yet in scope here
      let y = "Hello".to_string();
      let x = "World".to_string(); // <-- x's scope starts here...
      let h = "!".to_string(); //   |
   } //  <-------------- ...and ends here
   ```
2. ownership of the variable is transferred to someone else (e.g. a function or another variable)
   ```rust
   fn compute(t: String) {
      // Do something [...]
   }

   fn main() {
       let s = "Hello".to_string(); // <-- s's scope starts here...
                   //                    | 
       compute(s); // <------------------- ..and ends here
                   //   because `s` is moved into `compute`
   }
   ```

## Destructors

When the owner of a value goes out of scope, Rust invokes its **destructor**.\
The destructor tries to clean up the resources used by that value—in particular, whatever memory it allocated.

You can manually invoke the destructor of a value by passing it to `std::mem::drop`.\
That's why you'll often hear Rust developers saying "that value has been **dropped**" as a way to state that a value
has gone out of scope and its destructor has been invoked.

### Visualizing drop points

We can insert explicit calls to `drop` to "spell out" what the compiler does for us. Going back to the previous example:

```rust
fn main() {
   let y = "Hello".to_string();
   let x = "World".to_string();
   let h = "!".to_string();
}
```

It's equivalent to:

```rust
fn main() {
   let y = "Hello".to_string();
   let x = "World".to_string();
   let h = "!".to_string();
   // Variables are dropped in reverse order of declaration
   drop(h);
   drop(x);
   drop(y);
}
```

Let's look at the second example instead, where `s`'s ownership is transferred to `compute`:

```rust
fn compute(s: String) {
   // Do something [...]
}

fn main() {
   let s = "Hello".to_string();
   compute(s);
}
```

It's equivalent to this:

```rust
fn compute(t: String) {
    // Do something [...]
    drop(t); // <-- Assuming `t` wasn't dropped or moved 
             //     before this point, the compiler will call 
             //     `drop` here, when it goes out of scope
}

fn main() {
    let s = "Hello".to_string();
    compute(s);
}
```

Notice the difference: even though `s` is no longer valid after `compute` is called in `main`, there is no `drop(s)`
in `main`.
When you transfer ownership of a value to a function, you're also **transferring the responsibility of cleaning it up**.

This ensures that the destructor for a value is called **at most[^leak] once**, preventing
[double free bugs](https://owasp.org/www-community/vulnerabilities/Doubly_freeing_memory) by design.

### Use after drop

What happens if you try to use a value after it's been dropped?

```rust
let x = "Hello".to_string();
drop(x);
println!("{}", x);
```

If you try to compile this code, you'll get an error:

```rust
error[E0382]: use of moved value: `x`
 --> src/main.rs:4:20
  |
3 |     drop(x);
  |          - value moved here
4 |     println!("{}", x);
  |                    ^ value used here after move
```

Drop **consumes** the value it's called on, meaning that the value is no longer valid after the call.\
The compiler will therefore prevent you from using it, avoiding [use-after-free bugs](https://owasp.org/www-community/vulnerabilities/Using_freed_memory).

### Dropping references

What if a variable contains a reference?\
For example:

```rust
let x = 42i32;
let y = &x;
drop(y);
```

When you call `drop(y)`... nothing happens.\
If you actually try to compile this code, you'll get a warning:

```text
warning: calls to `std::mem::drop` with a reference 
         instead of an owned value does nothing
 --> src/main.rs:4:5
  |
4 |     drop(y);
  |     ^^^^^-^
  |          |
  |          argument has type `&i32`
  |
```

It goes back to what we said earlier: we only want to call the destructor once.\
You can have multiple references to the same value—if we called the destructor for the value they point at
when one of them goes out of scope, what would happen to the others?
They would refer to a memory location that's no longer valid: a so-called [**dangling pointer**](https://en.wikipedia.org/wiki/Dangling_pointer),
a close relative of [**use-after-free bugs**](https://owasp.org/www-community/vulnerabilities/Using_freed_memory).
Rust's ownership system rules out these kinds of bugs by design.

[^leak]: Rust doesn't guarantee that destructors will run. They won't, for example, if
you explicitly choose to [leak memory](../07_threads/03_leak.md).
