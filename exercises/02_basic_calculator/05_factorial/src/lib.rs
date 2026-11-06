// 非負の整数 `n` が与えられたときに、`n` の階乗である `n!` を返す
// `factorial` という名前の関数を定義してください。
//
// `n` の階乗は、`n` までのすべての正の整数の積として定義されます。
// 例えば、`5!`（「5の階乗」と読みます）は `5 * 4 * 3 * 2 * 1` であり、`120` になります。
// `0!` は `1` と定義されています。
//
// `factorial(0)` は `1` を返し、`factorial(1)` は `1` を返し、
// `factorial(2)` は `2` を返す、といった動作が期待されます。
//
// これまでに学んだことだけを使ってください！まだループは学んでいないので、再帰（recursion）を使う必要があります！

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
