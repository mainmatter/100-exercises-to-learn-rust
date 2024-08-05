# `'static`

If you tried to borrow a slice from the vector in the previous exercise,
you probably got a compiler error that looks something like this:

```text
error[E0597]: `v` does not live long enough
   |
11 | pub fn sum(v: Vec<i32>) -> i32 {
   |            - binding `v` declared here
...
15 |     let right = &v[split_point..];
   |                  ^ borrowed value does not live long enough
16 |     let left_handle = spawn(move || left.iter().sum::<i32>());
   |                             -------------------------------- 
                     argument requires that `v` is borrowed for `'static`
19 | }
   |  - `v` dropped here while still borrowed
```

`argument requires that v is borrowed for 'static`, what does that mean?

The `'static` lifetime is a special lifetime in Rust.\
It means that the value will be valid for the entire duration of the program.

## Detached threads

A thread launched via `thread::spawn` can **outlive** the thread that spawned it.\
For example:

```rust
use std::thread;

fn f() {
    thread::spawn(|| {
        thread::spawn(|| {
            loop {
                thread::sleep(std::time::Duration::from_secs(1));
                println!("Hello from the detached thread!");
            }
        });
    });
}
```

In this example, the first spawned thread will in turn spawn
a child thread that prints a message every second.\
The first thread will then finish and exit. When that happens,
its child thread will **continue running** for as long as the
overall process is running.\
In Rust's lingo, we say that the child thread has **outlived**
its parent.

## `'static` lifetime

Since a spawned thread can:

- outlive the thread that spawned it (its parent thread)
- run until the program exits

it must not borrow any values that might be dropped before the program exits;
violating this constraint would expose us to a use-after-free bug.\
That's why `std::thread::spawn`'s signature requires that the closure passed to it
has the `'static` lifetime:

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static
{
    // [..]
}
```

## `'static` is not (just) about references

All values in Rust have a lifetime, not just references.

In particular, a type that owns its data (like a `Vec` or a `String`)
satisfies the `'static` constraint: if you own it, you can keep working with it
for as long as you want, even after the function that originally created it
has returned.

You can thus interpret `'static` as a way to say:

- Give me an owned value
- Give me a reference that's valid for the entire duration of the program

The first approach is how you solved the issue in the previous exercise:
by allocating new vectors to hold the left and right parts of the original vector,
which were then moved into the spawned threads.

## `'static` references

Let's talk about the second case, references that are valid for the entire
duration of the program.

### Static data

The most common case is a reference to **static data**, such as string literals:

```rust
let s: &'static str = "Hello world!";
```

Since string literals are known at compile-time, Rust stores them _inside_ your executable,
in a region known as **read-only data segment**.
All references pointing to that region will therefore be valid for as long as
the program runs; they satisfy the `'static` contract.

## Further reading

- [The data segment](https://en.wikipedia.org/wiki/Data_segment)
