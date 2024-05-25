# Outro

Rust's asynchronous model is quite powerful, but it does introduce additional
complexity. Take time to know your tools: dive deep into `tokio`'s documentation
and get familiar with its primitives to make the most out of it.

Keep in mind, as well, that there is ongoing work at the language and `std` level
to streamline and "complete" Rust's asynchronous story. You may experience some
rough edges in your day-to-day work due to some of these missing pieces.

A few recommendations for a mostly-pain-free async experience:

- **Pick a runtime and stick to it.**\
  Some primitives (e.g. timers, I/O) are not portable across runtimes. Trying to
  mix runtimes is likely to cause you pain. Trying to write code that's runtime
  agnostic can significantly increase the complexity of your codebase. Avoid it
  if you can.
- **There is no stable `Stream`/`AsyncIterator` interface yet.**\
  An `AsyncIterator` is, conceptually, an iterator that yields new items
  asynchronously. There is ongoing design work, but no consensus (yet).
  If you're using `tokio`, refer to [`tokio_stream`](https://docs.rs/tokio-stream/latest/tokio_stream/)
  as your go-to interface.
- **Be careful with buffering.**\
  It is often the cause of subtle bugs. Check out
  ["Barbara battles buffered streams"](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/barbara_battles_buffered_streams.html)
  for more details.
- **There is no equivalent of scoped threads for asynchronous tasks**.\
  Check out ["The scoped task trilemma"](https://without.boats/blog/the-scoped-task-trilemma/)
  for more details.

Don't let these caveats scare you: asynchronous Rust is being used effectively
at _massive_ scale (e.g. AWS, Meta) to power foundational services.\
You will have to master it if you're planning building networked applications
in Rust.
