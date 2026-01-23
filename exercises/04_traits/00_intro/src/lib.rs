fn intro() -> &'static str {
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
