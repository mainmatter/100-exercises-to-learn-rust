# 可変参照 (Mutable references)

アクセサメソッドは次のようになっているはずです：

```rust
impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}
```

あちこちに `&` を少し散りばめるだけでうまくいきました！\
これで、`Ticket` インスタンスを消費することなく、そのフィールドにアクセスする方法が手に入りました。
次は、**セッターメソッド (setter methods)** を使って `Ticket` 構造体を強化する方法を見ていきましょう。

## セッター (Setters)

セッターメソッドを使用すると、`Ticket` の不変条件が守られていることを確認しながら（つまり、`Ticket` のタイトルを空文字列に設定できないようにしながら）、ユーザーが非公開フィールドの値を変更できるようになります。

Rustでセッターを実装するには、一般的な方法が2つあります：

- `self` を入力として受け取る方法
- `&mut self` を入力として受け取る方法

### `self` を入力として受け取る方法

最初のアプローチは次のようになります：

```rust
impl Ticket {
    pub fn set_title(mut self, new_title: String) -> Self {
        // 新しいタイトルを検証する [...]
        self.title = new_title;
        self
    }
}
```

これは `self` の所有権を奪い、タイトルを変更し、変更された `Ticket` インスタンスを返します。\
使い方は次のようになります：

```rust
let ticket = Ticket::new(
    "Title".into(), 
    "Description".into(), 
    "To-Do".into()
);
let ticket = ticket.set_title("New title".into());
```

`set_title` は `self` の所有権を奪う（つまり**消費する**）ため、結果を変数に再代入する必要があります。
上記の例では、同じ変数名を再利用するために**変数シャドーイング (variable shadowing)** を活用しています。既存の変数と同じ名前で新しい変数を宣言すると、新しい変数が古い変数を**覆い隠します (shadow)**。これはRustコードでよく見られるパターンです。

`self` を受け取るセッターは、一度に複数のフィールドを変更する必要がある場合に非常にうまく機能します。複数の呼び出しをメソッドチェーンでつなげることができるからです！

```rust
let ticket = ticket
    .set_title("New title".into())
    .set_description("New description".into())
    .set_status("In Progress".into());
```

### `&mut self` を入力として受け取る方法

`&mut self` を使用する2つ目のセッターのアプローチは、次のようになります：

```rust
impl Ticket {
    pub fn set_title(&mut self, new_title: String) {
        // 新しいタイトルを検証する [...]
        
        self.title = new_title;
    }
}
```

今回は、メソッドは入力として `self` への可変参照を受け取り、タイトルを変更し、それで終わりです。
何も返されません。

使い方は次のようになります：

```rust
let mut ticket = Ticket::new(
    "Title".into(),
    "Description".into(),
    "To-Do".into()
);
ticket.set_title("New title".into());

// 変更された ticket を使用する
```

所有権は呼び出し元に残るため、元の `ticket` 変数は引き続き有効です。結果を再代入する必要はありません。
ただし、`ticket` への可変参照を取得するため、`ticket` を `mut`（可変）としてマークする必要があります。

`&mut` セッターには欠点があります。複数の呼び出しをチェーンできないことです。
変更された `Ticket` インスタンスを返さないため、最初のセッターの結果に対して別のセッターを呼び出すことはできません。
各セッターを個別に呼び出す必要があります：

```rust
ticket.set_title("New title".into());
ticket.set_description("New description".into());
ticket.set_status("In Progress".into());
```
