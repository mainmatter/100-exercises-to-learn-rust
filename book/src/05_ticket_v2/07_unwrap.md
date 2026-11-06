# アンラップ (`unwrap`)

`Ticket::new` は、無効な入力に対してパニックするのではなく、`Result` を返すようになりました。\
これは呼び出し元にとって何を意味するのでしょうか？

## 失敗を（暗黙的に）無視することはできない

例外とは異なり、Rustの `Result` は**呼び出し箇所でエラーを処理する**ことを強制します。\
`Result` を返す関数を呼び出す場合、Rustはエラーケースを暗黙的に無視することを許可しません。

```rust
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}

// これはコンパイルできません。エラーケースを処理していないためです。
// 成功した値を「取り出す (unwrap)」かエラーを処理するためには、
// `match` または `Result` が提供するコンビネータの1つを使用する必要があります。
let number = parse_int("42") + 2;
```

## `Result` を受け取ったら、次はどうする？

`Result` を返す関数を呼び出したとき、主な選択肢は2つあります：

- 操作が失敗した場合にパニックさせる。
  これは `unwrap` または `expect` メソッドを使用して行います。
  ```rust
  // `parse_int` が `Err` を返した場合にパニックします。
  let number = parse_int("42").unwrap();
  // `expect` を使うと、カスタムのパニックメッセージを指定できます。
  let number = parse_int("42").expect("Failed to parse integer");
  ```
- `match` 式を使用して `Result` を分解（分割代入）し、エラーケースを明示的に処理する。
  ```rust
  match parse_int("42") {
      Ok(number) => println!("Parsed number: {}", number),
      Err(err) => eprintln!("Error: {}", err),
  }
  ```
