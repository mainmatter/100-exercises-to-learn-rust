# トレイトの実装とオーファンルール

別のクレートで定義されている型（例えば、Rustの標準ライブラリの `u32`）に対して、直接新しいメソッドを定義することはできません。もし次のように試みると：

```rust
impl u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
```

コンパイラはエラーを出します：

```text
error[E0390]: cannot define inherent `impl` for primitive types
  |
1 | impl u32 {
  | ^^^^^^^^
  |
  = help: consider using an extension trait instead
```

## 拡張トレイト (Extension trait)

**拡張トレイト (extension trait)** とは、`u32` のような外部の型（他のクレートで定義された型）に新しいメソッドを追加することを主な目的とするトレイトです。
前回の演習で `IsEven` トレイトを定義し、それを `i32` と `u32` に実装したのが、まさにそのパターンです。`IsEven` がスコープ内にある限り、それらの型に対して自由に `is_even` を呼び出すことができます。

```rust
// トレイトをスコープに持ち込む
use my_library::IsEven;

fn main() {
    // トレイトを実装している型に対してそのメソッドを呼び出す
    if 4.is_even() {
        // [...]
    }
}
```

## 実装は1つだけ

記述できるトレイト実装にはいくつかの制限があります。\
最も単純でわかりやすい制限は、1つのクレート内で、同じ型に対して同じトレイトを2回実装することはできないというものです。

例えば：

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        true
    }
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        false
    }
}
```

コンパイラはこれを拒否します：

```text
error[E0119]: conflicting implementations of trait `IsEven` for type `u32`
   |
5  | impl IsEven for u32 {
   | ------------------- first implementation here
...
11 | impl IsEven for u32 {
   | ^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u32`
```

`u32` の値に対して `IsEven::is_even` が呼び出されたとき、どのトレイト実装を使用すべきかについて曖昧さがあってはならないため、実装はただ1つしか存在できません。

## オーファンルール (Orphan rule)

複数のクレートが関与する場合、事態はもう少し複雑になります。
具体的には、少なくとも以下のいずれか一方が真でなければなりません：

- トレイトが現在のクレートで定義されている
- トレイトを実装する対象の型が現在のクレートで定義されている

これはRustの**オーファンルール (orphan rule / 孤児規則)** と呼ばれます。その目的は、メソッド解決のプロセスから曖昧さをなくすことです。

次のような状況を想像してみてください：

- クレート `A` が `IsEven` トレイトを定義する
- クレート `B` が `u32` に対して `IsEven` を実装する
- クレート `C` が `u32` に対して `IsEven` トレイトの（異なる）実装を提供する
- クレート `D` が `B` と `C` の両方に依存し、`1.is_even()` を呼び出す

どの実装を使用すべきでしょうか？`B` で定義されたものでしょうか、それとも `C` で定義されたものでしょうか？\
良い答えがないため、このシナリオを防ぐためにオーファンルールが定義されました。
オーファンルールのおかげで、クレート `B` もクレート `C` もコンパイルできません。

## さらに詳しく知りたい方へ

- 上記で述べたオーファンルールには、いくつかの注意点や例外があります。
  細かいニュアンスについて詳しく知りたい場合は、[リファレンス](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence)をチェックしてください。
