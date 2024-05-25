# Two-way communication

In our current client-server implementation, communication flows in one direction: from the client to the server.\
The client has no way of knowing if the server received the message, executed it successfully, or failed.
That's not ideal.

To solve this issue, we can introduce a two-way communication system.

## Response channel

We need a way for the server to send a response back to the client.\
There are various ways to do this, but the simplest option is to include a `Sender` channel in
the message that the client sends to the server. After processing the message, the server can use
this channel to send a response back to the client.

This is a fairly common pattern in Rust applications built on top of message-passing primitives.
