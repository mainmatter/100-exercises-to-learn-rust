# `Drop` トレイト

[デストラクタ](../03_ticket_v1/11_destructor.md)を紹介した際、`drop` 関数は以下のことを行うと述べました：

1. 型が占有していたメモリ（つまり `std::mem::size_of` バイト）を回収する
2. 値が管理している可能性のある追加のリソース（例：`String` のヒープバッファ）をクリーンアップする

ステップ 2 こそが、`Drop` トレイトの出番です。

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

`Drop` トレイトは、コンパイラが自動的に行ってくれる処理を超えて、型に対して*追加の*クリーンアップロジックを定義するための仕組みです。\
`drop` メソッド内に記述した処理は何であれ、その値がスコープを抜けたときに実行されます。

## `Drop` と `Copy`

`Copy` トレイトについて話した際、メモリ上で占有する `std::mem::size_of` バイト以外の追加リソースを管理している型は、`Copy` を実装できないと述べました。

ここで疑問に思うかもしれません。コンパイラはどうやって、ある型が追加のリソースを管理しているかどうかを知るのでしょうか？
その通りです、`Drop` トレイトの実装によって知るのです！\
自分の型に明示的な `Drop` の実装がある場合、コンパイラはその型に追加のリソースが紐づいているとみなし、`Copy` の実装を許可しません。

```rust
// これはユニット構造体（つまりフィールドを持たない構造体）です。
#[derive(Clone, Copy)]
struct MyType;

impl Drop for MyType {
    fn drop(&mut self) {
       // ここで何かをする必要はありません。
       // 「空の」Drop実装があるだけで十分です。
    }
}
```

コンパイラは次のエラーメッセージを出します：

```text
error[E0184]: the trait `Copy` cannot be implemented for this type; 
              the type has a destructor
 --> src/lib.rs:2:17
  |
2 | #[derive(Clone, Copy)]
  |                 ^^^^ `Copy` not allowed on types with destructors
```
