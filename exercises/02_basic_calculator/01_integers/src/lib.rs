fn compute(a: u32, b: u32) -> u32 {
    // TODO: 以下の行を変更してコンパイルエラーを修正し、テストが通るようにしてください。
    let multiplier: u8 = 4;
    a + b * multiplier
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
