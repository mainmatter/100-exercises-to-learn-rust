/// 移動の開始地点と終了地点、および移動にかかった時間が与えられたとき、
/// 移動の平均速度を計算します。
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: `time_elapsed` が 0 の場合、カスタムメッセージを出してパニックさせてください

    (end - start) / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    // 👇 `#[should_panic]` アノテーションを使うと、テスト対象のコードがパニックすることを
    //    期待しているとアサート（検証）できます。また、`expected` を使ってパニックメッセージを
    //    チェックすることもできます。これらはすべてRustの組み込みテストフレームワークの一部です！
    #[should_panic(expected = "The journey took no time at all. That's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
