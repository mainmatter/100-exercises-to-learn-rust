# 順序付け (`BTreeMap` と `Ord`)

`Vec` から `HashMap` に移行したことで、チケット管理システムのパフォーマンスが向上し、その過程でコードも簡素化されました。\
しかし、良いことばかりではありません。`Vec` をベースにしたストアを反復処理していたときは、チケットが追加された順序で返されることが確実でした。\
`HashMap` の場合はそうではありません。チケットを反復処理することはできますが、その順序はランダムになります。

`HashMap` から `BTreeMap` に切り替えることで、一貫した順序付けを取り戻すことができます。

## `BTreeMap`

`BTreeMap` は、エントリがそのキーによってソートされることを保証します。\
これは、特定の順序でエントリを反復処理する必要がある場合や、範囲クエリ（例：「IDが10から20の間のすべてのチケットを取得する」）を実行する必要がある場合に便利です。

`HashMap` と同様に、`BTreeMap` の定義自体にはトレイト境界はありません。
しかし、そのメソッドにはトレイト境界があります。`insert` を見てみましょう：

```rust
// `HashMap` と同様に、`K` と `V` はそれぞれ
// キーと値の型を表します。
impl<K, V> BTreeMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        // 実装
    }
}
```

`Hash` は不要になりました。その代わり、キーの型は `Ord` トレイトを実装していなければなりません。

## `Ord`

`Ord` トレイトは値を比較するために使用されます。\
`PartialEq` が等価性の比較に使用されるのに対し、`Ord` は大小関係（順序）の比較に使用されます。

これは `std::cmp` で定義されています：

```rust
pub trait Ord: Eq + PartialOrd {
    fn cmp(&self, other: &Self) -> Ordering;
}
```

`cmp` メソッドは `Ordering` 列挙型を返します。これは `Less`（より小さい）、`Equal`（等しい）、`Greater`（より大きい）のいずれかの値をとります。\
`Ord` は、他の2つのトレイトである `Eq` と `PartialOrd` が実装されていることを要求します。

## `PartialOrd`

`PartialEq` が `Eq` の弱いバージョンであるのと同様に、`PartialOrd` は `Ord` の弱いバージョン（部分順序）です。
その定義を見ると理由がわかります：

```rust
pub trait PartialOrd: PartialEq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
}
```

`PartialOrd::partial_cmp` は `Option` を返します。つまり、2つの値が常に比較可能であるとは保証されていません。\
例えば、`f32` が `Eq` を実装していないのと同じ理由（`NaN` の値が比較不能であるため）で、`f32` は `Ord` を実装していません。

## `Ord` と `PartialOrd` の実装

`Ord` と `PartialOrd` はどちらも、自分の型に対して `derive`（導出）することができます：

```rust
// `Ord` が要求するため、`Eq` と `PartialEq` も
// 追加する必要があります。
#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct TicketId(u64);
```

手動で実装することを選択した（またはその必要がある）場合は、以下の点に注意してください：

- `Ord` と `PartialOrd` は、`Eq` および `PartialEq` と一貫していなければなりません。
- `Ord` と `PartialOrd` は、互いに一貫していなければなりません。
