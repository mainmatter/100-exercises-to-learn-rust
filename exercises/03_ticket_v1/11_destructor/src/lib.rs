// デストラクタに関する適切な演習を作成するには、もう少し他の仕組みが必要です。
// トレイトと内部可変性を扱った後の章で、再びこの概念を取り上げます。
fn outro() -> &'static str {
    "I have a basic understanding of __!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a basic understanding of destructors!");
    }
}
