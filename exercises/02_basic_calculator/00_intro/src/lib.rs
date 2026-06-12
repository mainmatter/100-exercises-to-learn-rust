fn intro() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to build a calculator in Rust!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        let b:u8 = 32;
        assert_eq!(intro(), "I'm ready to build a calculator in Rust!");
    }


}
