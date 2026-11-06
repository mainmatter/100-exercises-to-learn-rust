# パニック (Panics)

[「変数」セクション](02_variables.md)で書いた `speed` 関数に戻りましょう。
おそらく次のようなコードだったはずです：

```rust
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    let distance = end - start;
    distance / time_elapsed
}
```

鋭い目をお持ちの方なら、1つ問題点[^one]に気づいたかもしれません。もし `time_elapsed` が 0 だったらどうなるでしょうか？

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=36e5ddbe3b3f741dfa9f74c956622bac) で実際に試してみることができます！\
プログラムは以下のエラーメッセージを出して終了します：

```text
thread 'main' panicked at src/main.rs:3:5:
attempt to divide by zero
```

これは**パニック (panic)** と呼ばれます。\
パニックとは、プログラムの実行を継続できないほど深刻な問題が発生したことを知らせるRustの仕組みであり、**回復不能なエラー (unrecoverable error)**[^catching] です。ゼロ除算はそのようなエラーに分類されます。

## `panic!` マクロ

`panic!` マクロ[^macro]を呼び出すことで、意図的にパニックを引き起こすことができます：

```rust
fn main() {
    panic!("This is a panic!");
    // 以下の行は決して実行されません
    let x = 1 + 2;
}
```

Rustには回復可能なエラーを扱うための他の仕組みもあり、それについては[後半で説明します](../05_ticket_v2/06_fallibility.md)。
当面の間は、乱暴ですがシンプルな一時しのぎの解決策としてパニックを使っていきます。

## さらに詳しく知りたい方へ

- [`panic!` マクロのドキュメント](https://doc.rust-lang.org/std/macro.panic.html)

[^one]: `speed` にはもう1つ問題があり、それについてもすぐ後で対処します。どこだかわかりますか？

[^catching]: パニックをキャッチ（捕捉）しようとすることも可能ですが、それは非常に特殊な状況に限定された最終手段とすべきです。

[^macro]: 名前の後ろに `!` が付いている場合、それはマクロ呼び出しです。今のところは、マクロを「ちょっと特別な関数」くらいに考えておいてください。コースの後半でより詳しく説明します。
