# 列挙型 (Enumerations)

[以前の章](../03_ticket_v1/02_validation.md)で書いたバリデーションロジックに基づくと、チケットに対して有効なステータスは `To-Do`、`InProgress`、`Done` のわずかしかありません。\
しかし、`Ticket` 構造体の `status` フィールドや、`new` メソッドの `status` パラメータの型を見ても、このことはすぐにはわかりません：

```rust
#[derive(Debug, PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: String
    ) -> Self {
        // [...]
    }
}
```

どちらのケースでも、`status` フィールドを表現するために `String` を使用しています。
`String` は非常に汎用的な型であり、`status` フィールドがとり得る値の集合が限られているという情報を即座には伝えてくれません。さらに悪いことに、`Ticket::new` の呼び出し元は、渡したステータスが有効かどうかを**実行時**になるまで知ることができません。

**列挙型 (enumerations)** を使えば、これをもっと改善できます。

## `enum`

列挙型とは、**バリアント (variants / 列挙子)** と呼ばれる固定された値の集合をとることができる型です。\
Rustでは、`enum` キーワードを使用して列挙型を定義します：

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

`enum` は、`struct` と同様に**新しいRustの型**を定義します。
