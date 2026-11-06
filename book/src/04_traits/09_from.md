# `From` と `Into`

文字列をめぐる旅が始まった場所に戻りましょう：

```rust
let ticket = Ticket::new(
    "A title".into(), 
    "A description".into(), 
    "To-Do".into()
);
```

ここで `.into()` が何をしているのかを紐解くのに十分な知識が揃いました。

## 問題点

`new` メソッドのシグネチャは次の通りです：

```rust
impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: String
    ) -> Self {
        // [...]
    }
}
```

また、文字列リテラル（`"A title"` など）の型は `&str` であることも見てきました。\
ここで型の不一致が生じています。`String` が期待されているのに、手元にあるのは `&str` です。
今回は魔法のような型強制が助けに来てくれることはありません。**型変換を実行する**必要があります。

## `From` と `Into`

Rustの標準ライブラリは、`std::convert` モジュール内で**失敗しない型変換 (infallible conversions)** のための2つのトレイト、`From` と `Into` を定義しています。

```rust
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    fn into(self) -> T;
}
```

これらのトレイト定義には、これまで見たことのない概念がいくつか含まれています。**スーパートレイト (supertraits)** と**暗黙のトレイト境界 (implicit trait bounds)** です。
まずはそれらを紐解いていきましょう。

### スーパートレイトとサブトレイト

`From: Sized` という構文は、`From` が `Sized` の**サブトレイト (subtrait)** であることを意味します。つまり、`From` を実装する任意の型は、`Sized` も実装していなければなりません。
逆に言えば、`Sized` は `From` の**スーパートレイト (supertrait)** であると言うこともできます。

### 暗黙のトレイト境界

ジェネリック型パラメータを導入するたびに、コンパイラは暗黙的にその型が `Sized` であると仮定します。

例えば：

```rust
pub struct Foo<T> {
    inner: T,
}
```

これは実際には以下と等価です：

```rust
pub struct Foo<T: Sized> 
{
    inner: T,
}
```

`From<T>` の場合、トレイト定義は以下と等価になります：

```rust
pub trait From<T: Sized>: Sized {
    fn from(value: T) -> Self;
}
```

言い換えれば、前者の境界は暗黙的であるものの、`T` と `From<T>` を実装する型の*両方*が `Sized` でなければなりません。

### 負のトレイト境界 (Negative trait bounds)

**負のトレイト境界 (negative trait bound)**（または境界の緩和）を使用すると、暗黙の `Sized` 境界を解除（オプトアウト）できます：

```rust
pub struct Foo<T: ?Sized> {
    //            ^^^^^^^
    //            これが負のトレイト境界です
    inner: T,
}
```

この構文は「`T` は `Sized` であるかもしれないし、そうでないかもしれない」と読み、`T` をDSTに束縛することを可能にします（例：`Foo<str>`）。ただし、これは特別なケースです。`?` を使った境界の緩和は `Sized` 専用であり、他のトレイトには使用できません。

## `&str` から `String` へ

[`std` のドキュメント](https://doc.rust-lang.org/std/convert/trait.From.html#implementors)を見ると、どの標準ライブラリの型が `From` トレイトを実装しているかを確認できます。\
そこでは、`String` が `From<&str> for String` を実装していることがわかります。したがって、次のように書くことができます：

```rust
let title = String::from("A title");
```

しかし、私たちは主に `.into()` を使用してきました。\
[`Into` の実装者一覧](https://doc.rust-lang.org/std/convert/trait.Into.html#implementors)を確認しても、`Into<String> for &str` は見当たりません。一体どうなっているのでしょうか？

`From` と `Into` は**双対 (dual) のトレイト**です。\
具体的には、**ブランケット実装 (blanket implementation / 一括実装)** を使用して、`From` を実装する任意の型に対して `Into` が自動的に実装されます：

```rust
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

型 `U` が `From<T>` を実装している場合、`Into<U> for T` が自動的に実装されます。だからこそ、`let title = "A title".into();` と書くことができるのです。

## `.into()`

`.into()` を見かけるたびに、そこでは型同士の変換が行われています。\
しかし、変換先の型（ターゲット型）は何なのでしょうか？

ほとんどの場合、変換先の型は以下のいずれかです：

- 関数やメソッドのシグネチャによって指定されている（例：上記の例の `Ticket::new`）
- 変数宣言の中で型注釈によって指定されている（例：`let title: String = "A title".into();`）

コンパイラが文脈から曖昧さなく変換先の型を推論できる限り、`.into()` はそのまま問題なく機能します。
