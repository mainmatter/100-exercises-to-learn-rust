# The `Future` trait

## The local `Rc` problem

Let's go back to `tokio::spawn`'s signature:

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
{ /* */ }
```

What does it _actually_ mean for `F` to be `Send`?\
It implies, as we saw in the previous section, that whatever value it captures from the
spawning environment has to be `Send`. But it goes further than that.

Any value that's _held across a .await point_ has to be `Send`.\
Let's look at an example:

```rust
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    // A value that's not `Send`,
    // created _inside_ the async function
    let non_send = Rc::new(1);
    
    // A `.await` point that does nothing
    yield_now().await;

    // The local non-`Send` value is still needed
    // after the `.await`
    println!("{}", non_send);
}
```

The compiler will reject this code:

```text
error: future cannot be sent between threads safely
    |
5   |     tokio::spawn(example());
    |                  ^^^^^^^^^ 
    | future returned by `example` is not `Send`
    |
note: future is not `Send` as this value is used across an await
    |
11  |     let non_send = Rc::new(1);
    |         -------- has type `Rc<i32>` which is not `Send`
12  |     // A `.await` point
13  |     yield_now().await;
    |                 ^^^^^ 
    |   await occurs here, with `non_send` maybe used later
note: required by a bound in `tokio::spawn`
    |
164 |     pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    |            ----- required by a bound in this function
165 |     where
166 |         F: Future + Send + 'static,
    |                     ^^^^ required by this bound in `spawn`
```

To understand why that's the case, we need to refine our understanding of
Rust's asynchronous model.

## The `Future` trait

We stated early on that `async` functions return **futures**, types that implement
the `Future` trait. You can think of a future as a **state machine**.
It's in one of two states:

- **pending**: the computation has not finished yet.
- **ready**: the computation has finished, here's the output.

This is encoded in the trait definition:

```rust
trait Future {
    type Output;
    
    // Ignore `Pin` and `Context` for now
    fn poll(
      self: Pin<&mut Self>, 
      cx: &mut Context<'_>
    ) -> Poll<Self::Output>;
}
```

### `poll`

The `poll` method is the heart of the `Future` trait.\
A future on its own doesn't do anything. It needs to be **polled** to make progress.\
When you call `poll`, you're asking the future to do some work.
`poll` tries to make progress, and then returns one of the following:

- `Poll::Pending`: the future is not ready yet. You need to call `poll` again later.
- `Poll::Ready(value)`: the future has finished. `value` is the result of the computation,
  of type `Self::Output`.

Once `Future::poll` returns `Poll::Ready`, it should not be polled again: the future has
completed, there's nothing left to do.

### The role of the runtime

You'll rarely, if ever, be calling poll directly.\
That's the job of your async runtime: it has all the required information (the `Context`
in `poll`'s signature) to ensure that your futures are making progress whenever they can.

## `async fn` and futures

We've worked with the high-level interface, asynchronous functions.\
We've now looked at the low-level primitive, the `Future trait`.

How are they related?

Every time you mark a function as asynchronous, that function will return a future.
The compiler will transform the body of your asynchronous function into a **state machine**:
one state for each `.await` point.

Going back to our `Rc` example:

```rust
use std::rc::Rc;
use tokio::task::yield_now;

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
```

The compiler would transform it into an enum that looks somewhat like this:

```rust
pub enum ExampleFuture {
    NotStarted,
    YieldNow(Rc<i32>),
    Terminated,
}
```

When `example` is called, it returns `ExampleFuture::NotStarted`. The future has never
been polled yet, so nothing has happened.\
When the runtime polls it the first time, `ExampleFuture` will advance until the next
`.await` point: it'll stop at the `ExampleFuture::YieldNow(Rc<i32>)` stage of the state
machine, returning `Poll::Pending`.\
When it's polled again, it'll execute the remaining code (`println!`) and
return `Poll::Ready(())`.

When you look at its state machine representation, `ExampleFuture`,
it is now clear why `example` is not `Send`: it holds an `Rc`, therefore
it cannot be `Send`.

## Yield points

As you've just seen with `example`, every `.await` point creates a new intermediate
state in the lifecycle of a future.\
That's why `.await` points are also known as **yield points**: your future _yields control_
back to the runtime that was polling it, allowing the runtime to pause it and (if necessary)
schedule another task for execution, thus making progress on multiple fronts concurrently.

We'll come back to the importance of yielding in a later section.
