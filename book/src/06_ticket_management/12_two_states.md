# チケットIDと2つの状態

チケット管理システムについてもう一度考えてみましょう。\
現在のチケットモデルは次のようになっています：

```rust
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

ここに1つ欠けているものがあります。チケットを一意に識別するための**識別子 (identifier / ID)** です。\
その識別子は各チケットに対して一意である必要があります。これは、新しいチケットが作成されるときにIDを自動生成することで保証できます。

## モデルの洗練

IDはどこに保存すべきでしょうか？\
`Ticket` 構造体に新しいフィールドを追加することが考えられます：

```rust
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

しかし、チケットを作成する前にはIDがわからないため、最初からそこに含めることはできません。\
そのため、オプショナル（任意）にする必要があります：

```rust
pub struct Ticket {
    pub id: Option<TicketId>,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

これも理想的とは言えません。チケットが一度作成されれば常にIDが存在するはずだとわかっているにもかかわらず、ストアからチケットを取得するたびに毎回 `None` のケースを処理しなければならなくなるからです。

最良の解決策は、`TicketDraft`（下書き）と `Ticket` という2つの別々の型で表現される、2つの異なるチケットの**状態 (states)** を持つことです：

```rust
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription
}

pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

`TicketDraft` はまだ作成（登録）されていないチケットです。IDを持たず、ステータスも持ちません。\
`Ticket` はすでに作成されたチケットです。IDとステータスを持っています。\
`TicketDraft` と `Ticket` の各フィールドにはそれぞれの制約が型として組み込まれているため、2つの型の間でバリデーションロジックを重複させる必要はありません。
