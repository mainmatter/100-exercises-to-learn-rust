# Wrapping up

We've covered quite a few different traits in this chapter—and we've only scratched the surface!
It may feel like you have a lot to remember, but don't worry: you'll bump into these traits
so often when writing Rust code that they'll soon become second nature.

## Closing thoughts

Traits are powerful, but don't overuse them.\
A few guidelines to keep in mind:

- Don't make a function generic if it is always invoked with a single type. It introduces indirection in your
  codebase, making it harder to understand and maintain.
- Don't create a trait if you only have one implementation. It's a sign that the trait is not needed.
- Implement standard traits for your types (`Debug`, `PartialEq`, etc.) whenever it makes sense.
  It will make your types more idiomatic and easier to work with, unlocking a lot of functionality provided
  by the standard library and ecosystem crates.
- Implement traits from third-party crates if you need the functionality they unlock within their ecosystem.
- Beware of making code generic solely to use mocks in your tests. The maintainability cost of this approach
  can be high, and it's often better to use a different testing strategy. Check out the
  [testing masterclass](https://github.com/mainmatter/rust-advanced-testing-workshop)
  for details on high-fidelity testing.

## Testing your knowledge

Before moving on, let's go through one last exercise to consolidate what we've learned.
You'll have minimal guidance this time—just the exercise description and the tests to guide you.
