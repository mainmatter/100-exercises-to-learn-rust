// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

/// A "drop bomb" is a type that will panic when it is dropped unless
/// it has been "defused" first. This is a useful pattern to enforce
/// that a certain cleanup action is performed.
#[derive(Default)]
pub struct DropBomb {
    // This flag tracks whether the bomb has been defused.
    // If it's `false` when the `DropBomb` is dropped, the program will panic.
    defused: bool,
}

impl DropBomb {
    /// Creates a new, "armed" `DropBomb`.
    pub fn new() -> Self {
        // The default value for `defused` is `false`, so the bomb is live.
        DropBomb::default()
    }

    /// Defuses the bomb, preventing it from panicking when dropped.
    pub fn defuse(&mut self) {
        self.defused = true;
    }
}

// This is the core of the "drop bomb" pattern.
// The `drop` method is automatically called by the Rust compiler
// when a `DropBomb` instance goes out of scope.
impl Drop for DropBomb {
    fn drop(&mut self) {
        // If the bomb hasn't been defused, we panic.
        // The `assert!` macro will panic if its argument is `false`.
        // In a real-world scenario, you might want a more descriptive panic message,
        // e.g., `if !self.defused { panic!("DropBomb went off!"); }`
        assert!(self.defused)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        // We create a new `DropBomb`...
        let _bomb = DropBomb::new();
        // ...and then let it go out of scope without calling `defuse()`.
        // The test framework expects this to panic, so the test passes.
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb is defused, so it should not panic when it goes out of scope
        // at the end of the function. The test passes if there is no panic.
    }
}
