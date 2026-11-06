# インデックスアクセス (`Index` トレイト)

`TicketStore::get` は、指定された `TicketId` に対して `Option<&Ticket>` を返します。\
以前、Rustのインデックス構文 (`[]`) を使って配列やベクタの要素にアクセスする方法を見ました：

```rust
let v = vec![0, 1, 2];
assert_eq!(v[0], 0);
```

`TicketStore` でも同じ使い勝手を提供するにはどうすればよいでしょうか？\
ご想像の通り、`Index` トレイトを実装する必要があります！

## `Index`

`Index` トレイトはRustの標準ライブラリで定義されています：

```rust
// 少し簡略化しています
pub trait Index<Idx>
{
    type Output;

    // 必須メソッド
    fn index(&self, index: Idx) -> &Self::Output;
}
```

これには以下の要素があります：

- インデックスの型を表す1つのジェネリックパラメータ `Idx`
- インデックスを使用して取得される型を表す1つの関連型 `Output`

`index` メソッドが `Option` を返さないことに注目してください。配列やベクタのインデックスアクセスと同様に、存在しない要素にアクセスしようとした場合、`index` はパニックすることが前提となっています。
