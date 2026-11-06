// オーバーフロー時にラップアラウンド（折り返し）するように `dev` プロファイルをカスタマイズしてください。
// 正しい構文についてはCargoのドキュメントを確認してください：
// https://doc.rust-lang.org/cargo/reference/profiles.html
//
// 後ほど説明する理由により、このカスタマイズは演習の `Cargo.toml` ではなく、
// リポジトリのルートにある `Cargo.toml` で行う必要があります。

pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        // 20! は 2432902008176640000 であり、u32 に収めるには大きすぎます。
        // デフォルトの dev プロファイルでは、`cargo test` を実行するとパニックになります。
        // ここでは代わりにラップアラウンドさせたいと考えています。
        assert_eq!(factorial(20), 2_192_834_560);
        //                           ☝️
        // 可読性を高めるためにアンダースコアを使った大きな数値リテラルです！
    }

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
