// TODO: `self` を `n` 乗する `power` メソッドを持つ新しいトレイト `Power` を定義してください。
//  トレイト定義とその実装だけで、テストがコンパイルされ通過するようになるはずです。
//
// 推奨事項：すべてのケースを一度に処理するためにジェネリックな実装を書きたくなるかもしれません。
// しかし、それはかなり複雑であり、追加のクレート（`num-traits` など）を使用する必要があります。
// 仮にそうしたとしても、高度にジェネリックな実装の複雑さを避けるためには、代わりにシンプルなマクロを
// 使用する方が望ましい場合もあります。これについて詳しく知りたい場合は、
// 「The Little Book of Rust Macros」(https://veykril.github.io/tlborm/) をチェックしてください。
// ただし、そうする必要はありません。3つの個別の実装を手動で書いてもまったく問題ありません。
// 好奇心がある場合にのみ、さらに踏み込んでみてください。

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
