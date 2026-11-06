// `self` が偶数の場合は `true` を返し、そうでない場合は `false` を返す
// `is_even` メソッドを持つ `IsEven` という名前のトレイトを定義してください。
//
// その後、`u32` と `i32` に対してそのトレイトを実装してください。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_is_even() {
        assert!(42u32.is_even());
        assert!(!43u32.is_even());
    }

    #[test]
    fn test_i32_is_even() {
        assert!(42i32.is_even());
        assert!(!43i32.is_even());
        assert!(0i32.is_even());
        assert!(!(-1i32).is_even());
    }
}
