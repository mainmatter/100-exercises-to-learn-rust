// `while` ループを使って `factorial` 関数を書き直してください。
pub fn factorial(n: u32) -> u32 {
    // `todo!()` マクロはプレースホルダーであり、コンパイラはこれを
    // 「後でここに戻って実装する」と解釈するため、型エラーを抑制します。
    // 実行時にはパニックを起こします。
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
