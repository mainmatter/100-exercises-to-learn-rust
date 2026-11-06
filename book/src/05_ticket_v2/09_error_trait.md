# `Error` トレイト

## エラー報告 (Error reporting)

前回の演習では、エラーメッセージを抽出して `panic!` マクロに渡すために、`TitleError` バリアントを分割代入する必要がありました。\
これは**エラー報告 (error reporting)**（エラー型をユーザー、サービス運用者、または開発者に表示できる表現に変換すること）の（初歩的な）一例です。

Rustの開発者が各自で独自のエラー報告戦略を考案するのは現実的ではありません。時間の無駄ですし、プロジェクト間でうまく組み合わせることができなくなってしまいます。
そのため、Rustは `std::error::Error` トレイトを提供しています。

## `Error` トレイト

`Result` の `Err` バリアントの型には何の制約もありませんが、`Error` トレイトを実装した型を使用するのが良いプラクティスとされています。
`Error` はRustのエラー処理の中核（コーナーストーン）です：

```rust
// 少し簡略化した `Error` トレイトの定義
pub trait Error: Debug + Display {}
```

[`From` トレイト](../04_traits/09_from.md#スーパートレイトとサブトレイト)で登場した `:` 構文を覚えているかもしれません。これは**スーパートレイト**を指定するために使用されます。
`Error` には `Debug` と `Display` という2つのスーパートレイトがあります。ある型が `Error` を実装したい場合、`Debug` と `Display` も実装しなければなりません。

## `Display` と `Debug`

`Debug` トレイトには[以前の演習](../04_traits/04_derive.md)ですでに出会っています。アサーションが失敗したときに、比較している変数の値を表示するために `assert_eq!` が使用するトレイトです。

「機械的な」観点から見ると、`Display` と `Debug` はまったく同じです。どちらも、型を文字列のような表現にどのように変換すべきかを定義しています：

```rust
// `Debug`
pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}

// `Display`
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

違いはその*目的*にあります。`Display` は「エンドユーザー」向けの表現を返すのに対し、`Debug` は開発者やサービス運用者に適した低レベルの表現を提供します。\
`Debug` が `#[derive(Debug)]` アトリビュートを使用して自動的に実装できるのに対し、`Display` が手動での実装を**必要とする**理由はそこにあります。
