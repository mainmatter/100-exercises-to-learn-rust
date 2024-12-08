// Trait bounds

/*
    Use cases:
    1) Unlocking 'build-in' behaviour (e.g. operator overloading)
    2) Adding new behaviour to existing types (i.e. extension traits)
    3) Generic programming

*/

fn intro() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to learn about traits!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to learn about traits!");
    }
}
