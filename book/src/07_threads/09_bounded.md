# Bounded vs unbounded channels

So far we've been using unbounded channels.\
You can send as many messages as you want, and the channel will grow to accommodate them.\
In a multi-producer single-consumer scenario, this can be problematic: if the producers
enqueue messages at a faster rate than the consumer can process them, the channel will
keep growing, potentially consuming all available memory.

Our recommendation is to **never** use an unbounded channel in a production system.\
You should always enforce an upper limit on the number of messages that can be enqueued using a
**bounded channel**.

## Bounded channels

A bounded channel has a fixed capacity.\
You can create one by calling `sync_channel` with a capacity greater than zero:

```rust
use std::sync::mpsc::sync_channel;

let (sender, receiver) = sync_channel(10);
```

`receiver` has the same type as before, `Receiver<T>`.\
`sender`, instead, is an instance of `SyncSender<T>`.

### Sending messages

You have two different methods to send messages through a `SyncSender`:

- `send`: if there is space in the channel, it will enqueue the message and return `Ok(())`.\
  If the channel is full, it will block and wait until there is space available.
- `try_send`: if there is space in the channel, it will enqueue the message and return `Ok(())`.\
  If the channel is full, it will return `Err(TrySendError::Full(value))`, where `value` is the message that couldn't be sent.

Depending on your use case, you might want to use one or the other.

### Backpressure

The main advantage of using bounded channels is that they provide a form of **backpressure**.\
They force the producers to slow down if the consumer can't keep up.
The backpressure can then propagate through the system, potentially affecting the whole architecture and
preventing end users from overwhelming the system with requests.
