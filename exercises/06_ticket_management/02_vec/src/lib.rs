// 数値 `n` が与えられたとき、フィボナッチ数列の `n+1` 番目の数値を返してください。
//
// フィボナッチ数列は次のように定義されます：
//
// - 数列の最初の数値は 0 です。
// - 数列の2番目の数値は 1 です。
// - それ以降の各数値は、直前の2つの数値の和になります。
//
// したがって、数列は 0, 1, 1, 2, 3, 5, 8, 13, 21... と続きます。
//
// `fibonacci(0)` は `0` を返し、`fibonacci(1)` は `1` を返し、
// `fibonacci(2)` は `1` を返す、といった動作が期待されます。
pub fn fibonacci(n: u32) -> u32 {
    // TODO: `fibonacci` 関数を実装してください
    //
    // ヒント：同じ計算を何度もやり直さなくて済むように、`Vec` を使って
    // すでに計算した結果をメモ化（保存）しましょう。
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
