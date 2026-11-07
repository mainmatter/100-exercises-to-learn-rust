# ループ (パート1): `while`

先ほどの `factorial` の実装では、再帰（recursion）を使わざるを得ませんでした。\
関数型プログラミングのバックグラウンドがある方にとっては自然に感じられたかもしれませんが、CやPythonのような命令型言語に慣れている方にとっては少し奇妙に感じられたかもしれません。

ここでは、代わりに**ループ (loop)** を使って同じ機能を実装する方法を見ていきましょう。

## `while` ループ

`while` ループは、**条件**が真 (`true`) である限りコードブロックを繰り返し実行する方法です。\
一般的な構文は以下の通りです：

```rust
while <condition> {
    // 実行するコード
}
```

例えば、1から5までの数値を合計したいとします：

```rust
let sum = 0;
let i = 1;
// 「i が 5 以下である間」
while i <= 5 {
    // `+=` は `sum = sum + i` の省略記法です
    sum += i;
    i += 1;
}
```

これは、`i` が 5 以下でなくなるまで、`i` に 1 を足し、`sum` に `i` を足し続けます。

## `mut` キーワード

上記の例は、そのままではコンパイルできません。以下のようなエラーが発生します：

```text
error[E0384]: cannot assign twice to immutable variable `sum`
 --> src/main.rs:7:9
  |
2 |     let sum = 0;
  |         ---
  |         |
  |         first assignment to `sum`
  |         help: consider making this binding mutable: `mut sum`
...
7 |         sum += i;
  |         ^^^^^^^^ cannot assign twice to immutable variable

error[E0384]: cannot assign twice to immutable variable `i`
 --> src/main.rs:8:9
  |
3 |     let i = 1;
  |         -
  |         |
  |         first assignment to `i`
  |         help: consider making this binding mutable: `mut i`
...
8 |         i += 1;
  |         ^^^^^^ cannot assign twice to immutable variable
```

これは、Rustの変数がデフォルトで**不変 (immutable)** であるためです。\
一度値が代入されると、その値を変更することはできません。

変更を許可したい場合は、`mut` キーワードを使って変数を**可変 (mutable)** として宣言する必要があります：

```rust
// これで `sum` と `i` は可変になりました！
let mut sum = 0;
let mut i = 1;

while i <= 5 {
    sum += i;
    i += 1;
}
```

これでエラーなくコンパイルおよび実行できるようになります。

## さらに詳しく知りたい方へ

- [`while` ループのドキュメント](https://doc.rust-lang.org/std/keyword.while.html)
