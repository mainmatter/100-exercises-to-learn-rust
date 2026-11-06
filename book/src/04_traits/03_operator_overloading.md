# 演算子のオーバーロード (Operator overloading)

トレイトとは何かについての基本的な理解が得られたところで、**演算子のオーバーロード (operator overloading)** に話を戻しましょう。
演算子のオーバーロードとは、`+`、`-`、`*`、`/`、`==`、`!=` などの演算子に対してカスタムの振る舞いを定義する機能のことです。

## 演算子はトレイトである

Rustでは、演算子はトレイトです。\
各演算子には、その演算子の振る舞いを定義する対応するトレイトが存在します。
自分の型に対してそのトレイトを実装することで、対応する演算子の使用が**解禁 (unlock)** されます。

例えば、[`PartialEq` トレイト](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)は `==` 演算子と `!=` 演算子の振る舞いを定義します：

```rust
// Rust標準ライブラリからの `PartialEq` トレイト定義
// （今のところは*少しだけ*簡略化しています）
pub trait PartialEq {
    // 必須メソッド (Required method)
    //
    // `Self` は「トレイトを実装している型」を表す
    // Rustのキーワードです
    fn eq(&self, other: &Self) -> bool;

    // 提供メソッド (Provided method)
    fn ne(&self, other: &Self) -> bool { ... }
}
```

`x == y` と書くと、コンパイラは `x` と `y` の型に対する `PartialEq` トレイトの実装を探し、`x == y` を `x.eq(y)` に置き換えます。これは糖衣構文（シンタックスシュガー）なのです！

主要な演算子とトレイトの対応関係は以下の通りです：

| 演算子                   | トレイト                                                                |
| ------------------------ | ----------------------------------------------------------------------- |
| `+`                      | [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html)               |
| `-`                      | [`Sub`](https://doc.rust-lang.org/std/ops/trait.Sub.html)               |
| `*`                      | [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html)               |
| `/`                      | [`Div`](https://doc.rust-lang.org/std/ops/trait.Div.html)               |
| `%`                      | [`Rem`](https://doc.rust-lang.org/std/ops/trait.Rem.html)               |
| `==` および `!=`         | [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)   |
| `<`、`>`、`<=`、および `>=` | [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) |

算術演算子は [`std::ops`](https://doc.rust-lang.org/std/ops/index.html) モジュールにあり、比較演算子は [`std::cmp`](https://doc.rust-lang.org/std/cmp/index.html) モジュールにあります。

## デフォルト実装 (Default implementations)

`PartialEq::ne` のコメントには「`ne` は提供メソッド (provided method) である」と書かれています。\
これは、`PartialEq` がトレイト定義の中で `ne` の**デフォルト実装 (default implementation)** を提供していることを意味します（定義スニペットで `{ ... }` と省略されているブロックです）。\
省略されているブロックを展開すると、次のようになります：

```rust
pub trait PartialEq {
    fn eq(&self, other: &Self) -> bool;

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
```

期待通り、`ne` は `eq` の否定になっています。\
デフォルト実装が提供されているため、自分の型に対して `PartialEq` を実装する際には `ne` の実装を省略できます。
`eq` を実装するだけで十分です：

```rust
struct WrappingU8 {
    inner: u8,
}

impl PartialEq for WrappingU8 {
    fn eq(&self, other: &WrappingU8) -> bool {
        self.inner == other.inner
    }
    
    // ここには `ne` の実装はありません
}
```

ただし、デフォルト実装の使用が強制されるわけではありません。
トレイトを実装する際に、それをオーバーライド（上書き）することを選択できます：

```rust
struct MyType;

impl PartialEq for MyType {
    fn eq(&self, other: &MyType) -> bool {
        // カスタム実装
    }

    fn ne(&self, other: &MyType) -> bool {
        // カスタム実装
    }
}
```
