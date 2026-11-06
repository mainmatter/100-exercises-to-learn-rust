# 簡潔な条件分岐 (`if let` と `let/else`)

前回の演習の解答は、おそらく次のようになっていることでしょう：

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        match &self.status {
            Status::InProgress { assigned_to } => assigned_to,
            Status::Done | Status::ToDo => {
                panic!(
                    "Only `In-Progress` tickets can be \
                    assigned to someone"
                )
            }
        }
    }
}
```

ここで関心があるのは `Status::InProgress` バリアントだけです。
本当に他のすべてのバリアントに対してマッチを行う必要があるのでしょうか？

新しい構文の出番です！

## `if let`

`if let` 構文を使用すると、他のすべてのバリアントを個別に処理することなく、列挙型の単一のバリアントに対してマッチを行うことができます。

`if let` を使って `assigned_to` メソッドを簡略化する方法は次の通りです：

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        if let Status::InProgress { assigned_to } = &self.status {
            assigned_to
        } else {
            panic!(
                "Only `In-Progress` tickets can be assigned to someone"
            );
        }
    }
}
```

## `let/else`

`else` 分岐が早期リターンを目的としている場合（パニックも早期リターンに数えられます！）、`let/else` 構文を使用できます：

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        let Status::InProgress { assigned_to } = &self.status else {
            panic!(
                "Only `In-Progress` tickets can be assigned to someone"
            );
        };
        assigned_to
    }
}
```

これを使うと、ネストが深くなってコードが右にずれていくのを防ぎながら、分割代入された変数を束縛できます。つまり、直前のコードと同じインデントレベルで変数が定義されます。

## スタイル

`if let` と `let/else` はどちらもイディオマティックなRustの構文です。\
コードの可読性を向上させるために適宜使用するとよいですが、無理に使いすぎないようにしましょう。必要なときにはいつでも `match` があります。
