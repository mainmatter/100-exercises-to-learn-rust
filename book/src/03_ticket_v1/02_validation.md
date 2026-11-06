# バリデーション (Validation)

チケットの定義に戻りましょう：

```rust
struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

ここでは、`Ticket` 構造体のフィールドに「生の（汎用の）」型を使用しています。
つまり、ユーザーは空のタイトル、とてつもなく長い説明文、あるいは意味不明なステータス（例：「Funny」）を持つチケットを作成できてしまいます。\
もっと良くできるはずです！

## さらに詳しく知りたい方へ

- `String` が提供するメソッドの全体像については、[`String` のドキュメント](https://doc.rust-lang.org/std/string/struct.String.html)をチェックしてください。今回の演習で必要になります！
