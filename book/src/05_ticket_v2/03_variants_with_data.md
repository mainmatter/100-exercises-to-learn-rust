# バリアントはデータを保持できる

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

私たちの `Status` 列挙型は、通常**C言語風の列挙型 (C-style enum)** と呼ばれるものです。\
各バリアントは単純なラベルであり、名前付き定数のようなものです。この種の列挙型は、C、C++、Java、C#、Pythonなど、多くのプログラミング言語で見られます。

しかし、Rustの列挙型はさらに一歩進んでいます。**各バリアントにデータを紐づける**ことができるのです。

## バリアント

例えば、現在チケットに取り組んでいる担当者の名前を保存したいとしましょう。\
この情報が存在するのは、チケットが進行中 (`InProgress`) の場合だけです。未着手 (`ToDo`) のチケットや完了 (`Done`) したチケットには存在しません。
これをモデル化するには、`InProgress` バリアントに `String` フィールドを紐づけます：

```rust
enum Status {
    ToDo,
    InProgress {
        assigned_to: String,
    },
    Done,
}
```

これで、`InProgress` は**構造体風のバリアント (struct-like variant)** になりました。\
実際、この構文は構造体を定義するときに使った構文そのものであり、列挙型の内部にバリアントとして「インライン化」されているだけです。

## バリアントのデータへのアクセス

`Status` インスタンスに対して直接 `assigned_to` にアクセスしようとすると：

```rust
let status: Status = /* */;

// これはコンパイルできません
println!("Assigned to: {}", status.assigned_to);
```

コンパイラがそれを止めます：

```text
error[E0609]: no field `assigned_to` on type `Status`
 --> src/main.rs:5:40
  |
5 |     println!("Assigned to: {}", status.assigned_to);
  |                                        ^^^^^^^^^^^ unknown field
```

`assigned_to` は**バリアント固有 (variant-specific)** であり、すべての `Status` インスタンスで利用できるわけではありません。\
`assigned_to` にアクセスするには、**パターンマッチング (pattern matching)** を使用する必要があります：

```rust
match status {
    Status::InProgress { assigned_to } => {
        println!("Assigned to: {}", assigned_to);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```

## 束縛 (Bindings)

マッチパターン `Status::InProgress { assigned_to }` における `assigned_to` は**束縛 (binding)** です。\
`Status::InProgress` バリアントを**分割代入 (destructuring)** し、`assigned_to` フィールドを同じく `assigned_to` という名前の新しい変数に束縛しています。\
必要であれば、フィールドを別の変数名に束縛することもできます：

```rust
match status {
    Status::InProgress { assigned_to: person } => {
        println!("Assigned to: {}", person);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```
