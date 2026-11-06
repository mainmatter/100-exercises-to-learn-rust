# 可変インデックスアクセス (`IndexMut` トレイト)

`Index` は読み取り専用のアクセスを許可しますが、取得した値を変更（ミューテート）することはできません。

## `IndexMut`

可変性を許可したい場合は、`IndexMut` トレイトを実装する必要があります。

```rust
// 少し簡略化しています
pub trait IndexMut<Idx>: Index<Idx>
{
    // 必須メソッド
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}
```

`IndexMut` は*追加の*機能を解禁するものであるため、その型がすでに `Index` を実装している場合にのみ実装できます。
