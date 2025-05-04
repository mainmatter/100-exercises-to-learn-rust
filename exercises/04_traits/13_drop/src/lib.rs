pub struct DropBomb {
    defused: bool,
}

impl DropBomb {
    pub fn new() -> Self {
        DropBomb { defused: false }
    }

    pub fn defuse(&mut self) {
        self.defused = true;
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.defused {
            panic!("Bomb dropped!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Bomb dropped!")]
    fn test_drop_bomb() {
        {
            let _bomb = DropBomb::new();
            // Bomb goes out of scope here and should panic
        }
    }

    #[test]
    fn test_no_panic() {
        let result = std::panic::catch_unwind(|| {
            let mut bomb = DropBomb::new();
            bomb.defuse();
        });
        assert!(result.is_ok(), "Expected no panic");
    }
}
