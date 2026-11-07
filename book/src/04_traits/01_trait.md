# トレイト (Traits)

もう一度 `Ticket` 型を見てみましょう：

```rust
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

これまでのすべてのテストでは、`Ticket` のフィールドを使ってアサーション（検証）を行っていました。

```rust
assert_eq!(ticket.title(), "A new title");
```

もし、2つの `Ticket` インスタンスを直接比較したいとしたらどうでしょうか？

```rust
let ticket1 = Ticket::new(/* ... */);
let ticket2 = Ticket::new(/* ... */);
ticket1 == ticket2
```

コンパイラがそれを止めます：

```text
error[E0369]: binary operation `==` cannot be applied to type `Ticket`
  --> src/main.rs:18:13
   |
18 |     ticket1 == ticket2
   |     ------- ^^ ------- Ticket
   |     |
   |     Ticket
   |
note: an implementation of `PartialEq` might be missing for `Ticket`
```

`Ticket` は新しい型です。そのままの状態では、**いかなる振る舞いも紐づいていません**。\
`String` を含んでいるからといって、Rustが2つの `Ticket` インスタンスを比較する方法を魔法のように推論してくれることはありません。

それでも、Rustコンパイラは私たちを正しい方向へ導いてくれています。`PartialEq` の実装が欠けている可能性を示唆しているのです。この `PartialEq` こそが**トレイト (trait)** です！

## トレイトとは何か？

トレイトとは、Rustにおける**インターフェース (interfaces)** の定義方法です。\
トレイトは、そのトレイトの契約（要件）を満たすために型が実装しなければならない一連のメソッドを定義します。

### トレイトの定義

トレイト定義の構文は次のようになります：

```rust
trait <TraitName> {
    fn <method_name>(<parameters>) -> <return_type>;
}
```

例えば、実装する型（実装者）に対して `is_zero` メソッドの定義を要求する `MaybeZero` という名前のトレイトを定義するとしましょう：

```rust
trait MaybeZero {
    fn is_zero(self) -> bool;
}
```

### トレイトの実装

型に対してトレイトを実装するには、通常の[^inherent]メソッドと同様に `impl` キーワードを使用しますが、構文が少し異なります：

```rust
impl <TraitName> for <TypeName> {
    fn <method_name>(<parameters>) -> <return_type> {
        // メソッド本体
    }
}
```

例えば、カスタム数値型である `WrappingU32` に対して `MaybeZero` トレイトを実装する場合は次のようになります：

```rust
pub struct WrappingU32 {
    inner: u32,
}

impl MaybeZero for WrappingU32 {
    fn is_zero(self) -> bool {
        self.inner == 0
    }
}
```

### トレイトメソッドの呼び出し

トレイトメソッドを呼び出すには、通常のメソッドと同様に `.` 演算子を使用します：

```rust
let x = WrappingU32 { inner: 5 };
assert!(!x.is_zero());
```

トレイトメソッドを呼び出すには、以下の2つが成り立っている必要があります：

- 型がそのトレイトを実装していること。
- そのトレイトがスコープ内にあること。

後者を満たすために、トレイトに対する `use` 文を追加する必要がある場合があります：

```rust
use crate::MaybeZero;
```

ただし、以下の場合には `use` 文は必要ありません：

- 呼び出しが行われるのと同じモジュール内でトレイトが定義されている場合。
- 標準ライブラリの**プレリュード (prelude)** でトレイトが定義されている場合。
  プレリュードとは、すべてのRustプログラムに自動的にインポートされる一連のトレイトと型のことです。
  すべてのRustモジュールの先頭に `use std::prelude::*;` が追加されているかのように動作します。

プレリュードに含まれているトレイトと型のリストは、[Rustのドキュメント](https://doc.rust-lang.org/std/prelude/index.html)で確認できます。

[^inherent]: トレイトを使用せずに型に直接定義されたメソッドは、**固有メソッド (inherent method)** とも呼ばれます。
