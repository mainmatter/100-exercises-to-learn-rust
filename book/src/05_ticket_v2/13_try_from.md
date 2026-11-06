# `TryFrom` と `TryInto`

前の章では、**失敗しない (infallible)** 型変換のためのRustのイディオマティックなインターフェースである [`From` トレイトと `Into` トレイト](../04_traits/09_from.md)を見ました。\
では、変換が成功する保証がない場合はどうすればよいでしょうか？

エラーについて十分な知識が得られたので、`From` と `Into` の**失敗する可能性のある (fallible)** 対応物である `TryFrom` と `TryInto` について説明しましょう。

## `TryFrom` と `TryInto`

`TryFrom` と `TryInto` はどちらも、`From` や `Into` と同様に `std::convert` モジュールで定義されています。

```rust
pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```

`From`/`Into` と `TryFrom`/`TryInto` の主な違いは、後者が `Result` 型を返す点です。\
これにより、変換が失敗する可能性を許容し、パニックする代わりにエラーを返すことができます。

## `Self::Error`

`TryFrom` と `TryInto` はどちらも関連型 `Error` を持っています。
これにより、各実装は独自のエラー型（理想的には、試みている変換に最も適したエラー型）を指定できます。

`Self::Error` は、トレイト自身で定義されている関連型 `Error` を参照するための書き方です。

## 双対性 (Duality)

`From` と `Into` とまったく同じように、`TryFrom` と `TryInto` も双対のトレイトです。\
ある型に対して `TryFrom` を実装すれば、自動的に `TryInto` も手に入ります。
