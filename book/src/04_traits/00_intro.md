# Traits

In the previous chapter we covered the basics of Rust's type and ownership system.\
It's time to dig deeper: we'll explore **traits**, Rust's take on interfaces.

Once you learn about traits, you'll start seeing their fingerprints all over the place.\
In fact, you've already seen traits in action throughout the previous chapter, e.g. `.into()` invocations as well
as operators like `==` and `+`.

On top of traits as a concept, we'll also cover some of the key traits that are defined in Rust's standard library:

- Operator traits (e.g. `Add`, `Sub`, `PartialEq`, etc.)
- `From` and `Into`, for infallible conversions
- `Clone` and `Copy`, for copying values
- `Deref` and deref coercion
- `Sized`, to mark types with a known size
- `Drop`, for custom cleanup logic

Since we'll be talking about conversions, we'll seize the opportunity to plug some of the "knowledge gaps"
from the previous chapter—e.g. what is `"A title"`, exactly? Time to learn more about slices too!
