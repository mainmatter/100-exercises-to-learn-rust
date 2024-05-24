# A dedicated `Client` type

All the interactions from the client side have been fairly low-level: you have to
manually create a response channel, build the command, send it to the server, and
then call `recv` on the response channel to get the response.

This is a lot of boilerplate code that could be abstracted away, and that's
exactly what we're going to do in this exercise.
