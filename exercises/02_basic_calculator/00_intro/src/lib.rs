fn intro() -> &'static str {
    // TODO: 修正してください 👇
    "I'm ready to __!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to build a calculator in Rust!");
    }
}
