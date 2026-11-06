# `match`

列挙型を使って実際に何が**できる**のだろうかと疑問に思っているかもしれません。\
最も一般的な操作は、列挙型に対して**マッチ (match)** を行うことです。

```rust
enum Status {
    ToDo,
    InProgress,
    Done
}

impl Status {
    fn is_done(&self) -> bool {
        match self {
            Status::Done => true,
            // `|` 演算子を使うと、複数のパターンにマッチさせることができます。
            // これは「`Status::ToDo` または `Status::InProgress` のいずれか」と読みます。
            Status::InProgress | Status::ToDo => false
        }
    }
}
```

`match` 式を使用すると、Rustの値を一連の**パターン (patterns)** と比較できます。\
これは型レベルの `if` のようなものだと考えることができます。`status` が `Done` バリアントであれば最初のブロックを実行し、`InProgress` または `ToDo` バリアントであれば2番目のブロックを実行します。

## 網羅性 (Exhaustiveness)

ここで1つ重要な点があります。`match` は**網羅的 (exhaustive)** です。すべての列挙型のバリアントを処理しなければなりません。\
バリアントの処理を1つでも忘れると、Rustは**コンパイル時に**エラーを出して止めてくれます。

例えば、`ToDo` バリアントの処理を忘れた場合：

```rust
match self {
    Status::Done => true,
    Status::InProgress => false,
}
```

コンパイラは次のようにエラーを出します：

```text
error[E0004]: non-exhaustive patterns: `ToDo` not covered
 --> src/main.rs:5:9
  |
5 |     match status {
  |     ^^^^^^^^^^^^ pattern `ToDo` not covered
```

これは非常に大きな利点です！\
コードベースは時間とともに進化します。将来、例えば `Blocked` のような新しいステータスを追加するかもしれません。するとRustコンパイラは、その新しいバリアントに対するロジックが欠けているすべての `match` 式に対してエラーを出力してくれます。
Rust開発者がしばしば「コンパイラ駆動リファクタリング」を称賛するのはこのためです。次に何をすべきかをコンパイラが教えてくれるので、報告された箇所を修正していくだけでよいのです。

## すべてを捕捉するパターン (Catch-all)

1つ以上のバリアントに関心がない場合は、すべてを捕捉する（キャッチオール）パターンとして `_` を使用できます：

```rust
match status {
    Status::Done => true,
    _ => false
}
```

`_` パターンは、それ以前のパターンにマッチしなかったすべてのものにマッチします。

<div class="warning">
  `_` の便利さには代償が伴います。列挙型が変更されたときに、コンパイラが助けてくれなくなるのです。上記の例で `Status` に `Blocked` バリアントが追加された場合、それが望ましい振る舞いであるかどうかにかかわらず、新しい `Blocked` バリアントは `_` のアーム（分岐）によって暗黙のうちに処理されるため、この `match` はそのままコンパイルが通ってしまいます。

正確さが重要な場面では、たとえ複数のアームが同じロジックを共有する場合であっても（例：`Status::ToDo | Status::InProgress => false`）、すべてのバリアントを明示的に書き出しましょう。そうすることで、新しいバリアントが追加された際に、必ず意識的な判断が求められるようになります。

</div>
