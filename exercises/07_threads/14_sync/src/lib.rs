// `Sync` については特に演習する内容はないので、覚えておくべきことの確認だけです。
fn outro() -> &'static str {
    "I have a good understanding of __!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a good understanding of Send and Sync!");
    }
}
