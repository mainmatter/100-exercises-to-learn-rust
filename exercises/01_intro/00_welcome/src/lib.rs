// This is a Rust file. It is a plain text file with a `.rs` extension.
//
// Like most modern programming languages, Rust supports comments. You're looking at one right now!
// Comments are ignored by the compiler; you can leverage them to annotate code with notes and
// explanations.
// There are various ways to write comments in Rust, each with its own purpose.
// For now we'll stick to the most common one: the line comment.
// Everything from `//` to the end of the line is considered a comment.

// Exercises will include `TODO`, `todo!()` or `__` markers to draw your attention to the lines
// where you need to write code.
// You'll need to replace these markers with your own code to complete the exercise.
// Sometimes it'll be enough to write a single line of code, other times you'll have to write
// longer sections.
//
// If you get stuck for more than 10 minutes on an exercise, grab a trainer! We're here to help!
// You can also find solutions to all exercises in the `solutions` git branch.
fn greeting() -> &'static str {
    "I'm ready to learn Rust!"
}

// Your solutions will be automatically verified by a set of tests.
// You can run these tests directly by invoking the `cargo test` command in your terminal,
// from the root of this exercise's directory. That's what the `wr` command does for you
// under the hood.
//
// Rust lets you write tests alongside your code.
// The `#[cfg(test)]` attribute tells the compiler to only compile the code below when
// running tests (i.e. when you run `cargo test`).
// You'll learn more about attributes and testing later in the course.
// For now, just know that you need to look for the `#[cfg(test)]` attribute to find the tests
// that will be verifying the correctness of your solutions!
//
// ⚠️ **DO NOT MODIFY THE TESTS** ⚠️
// They are there to help you validate your solutions. You should only change the code that's being
// tested, not the tests themselves.
#[cfg(test)]
mod tests {
    use crate::greeting;

    #[test]
    fn test_welcome() {
        assert_eq!(greeting(), "I'm ready to learn Rust!");
    }
}
