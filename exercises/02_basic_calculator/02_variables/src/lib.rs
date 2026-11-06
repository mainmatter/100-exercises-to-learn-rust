// 👇 `///` で始まる以下の行は、**ドキュメンテーションコメント (documentation comments)** と呼ばれます。
//    これらは直後に続くアイテム（この場合は `speed` 関数）にドキュメントを付与します。
//    この演習のディレクトリから `cargo doc --open` を実行すると、Rustはこれらのコメントから
//    HTMLドキュメントを生成し、ブラウザで開きます。

/// 移動の開始地点と終了地点、および移動にかかった時間が与えられたとき、
/// 平均速度を計算します。
pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: テストが通るように、正しい値を持つ `distance` という名前の変数を定義してください
    //  `distance` に型注釈を付ける必要はありますか？その理由はなぜですか？

    // 以下の行は変更しないでください
    distance / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(speed(10, 30, 10), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(speed(10, 31, 10), 2);
    }
}
