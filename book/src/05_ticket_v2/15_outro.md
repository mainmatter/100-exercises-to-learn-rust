# Wrapping up

When it comes to domain modelling, the devil is in the details.\
Rust offers a wide range of tools to help you represent the constraints of your domain directly in the type system,
but it takes some practice to get it right and write code that looks idiomatic.

Let's close the chapter with one final refinement of our `Ticket` model.\
We'll introduce a new type for each of the fields in `Ticket` to encapsulate the respective constraints.\
Every time someone accesses a `Ticket` field, they'll get back a value that's guaranteed to be valid—i.e. a
`TicketTitle` instead of a `String`. They won't have to worry about the title being empty elsewhere in the code:
as long as they have a `TicketTitle`, they know it's valid **by construction**.

This is just an example of how you can use Rust's type system to make your code safer and more expressive.

## Further reading

- [Parse, don't validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
- [Using types to guarantee domain invariants](https://www.lpalmieri.com/posts/2020-12-11-zero-to-production-6-domain-modelling/)
