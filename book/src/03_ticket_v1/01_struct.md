# 構造体 (Structs)

各チケットについて、以下の3つの情報を記録する必要があります：

- タイトル (title)
- 説明 (description)
- ステータス (status)

まずは、これらを表現するために [`String`](https://doc.rust-lang.org/std/string/struct.String.html) を使うところから始めましょう。`String` は、[UTF-8エンコードされた](https://en.wikipedia.org/wiki/UTF-8)テキストを表現するためにRustの標準ライブラリで定義されている型です。

では、これら3つの情報をどのようにして1つのエンティティ（実体）に**まとめる**のでしょうか？

## `struct` の定義

`struct`（構造体）は**新しいRustの型**を定義します。

```rust
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

構造体は、他のプログラミング言語でクラスやオブジェクトと呼ばれるものに非常に似ています。

## フィールドの定義

新しい型は、他の型を**フィールド (fields)** として組み合わせることで構築されます。\
各フィールドには名前と型が必要で、それらはコロン `:` で区切られます。複数のフィールドがある場合は、カンマ `,` で区切ります。

以下の `Configuration` 構造体に見られるように、フィールドはすべて同じ型である必要はありません：

```rust
struct Configuration {
   version: u32,
   active: bool
}
```

## インスタンス化 (Instantiation)

各フィールドの値を指定することで、構造体のインスタンスを作成できます：

```rust
// 構文: <構造体名> { <フィールド名>: <値>, ... }
let ticket = Ticket {
    title: "Build a ticket system".into(),
    description: "A Kanban board".into(),
    status: "Open".into()
};
```

## フィールドへのアクセス

`.` 演算子を使用して、構造体のフィールドにアクセスできます：

```rust
// フィールドアクセス
let x = ticket.description;
```

## メソッド (Methods)

**メソッド (methods)** を定義することで、構造体に振る舞いを紐づけることができます。\
`Ticket` 構造体を例にとってみましょう：

```rust
impl Ticket {
    fn is_open(self) -> bool {
        self.status == "Open"
    }
}

// 構文:
// impl <構造体名> {
//    fn <メソッド名>(<パラメータ>) -> <戻り値の型> {
//        // メソッド本体
//    }
// }
```

メソッドは関数と非常に似ていますが、2つの重要な違いがあります：

1. メソッドは **`impl` ブロック**の内部で定義しなければならない。
2. メソッドは最初のパラメータとして `self` を使用できる。
   `self` はキーワードであり、メソッドが呼び出されている構造体のインスタンスを表します。

### `self`

メソッドが最初のパラメータとして `self` を受け取る場合、**メソッド呼び出し構文 (method call syntax)** を使用して呼び出すことができます：

```rust
// メソッド呼び出し構文: <インスタンス>.<メソッド名>(<パラメータ>)
let is_open = ticket.is_open();
```

これは、[前の章](../02_basic_calculator/09_saturating.md)で `u32` の値に対して飽和演算を実行したときに使用した呼び出し構文と同じです。

### 静的メソッド (Static methods)

メソッドが最初のパラメータとして `self` を受け取らない場合、それは**静的メソッド (static method)**（または関連関数）です。

```rust
struct Configuration {
    version: u32,
    active: bool
}

impl Configuration {
    // `default` は `Configuration` の静的メソッドです
    fn default() -> Configuration {
        Configuration { version: 0, active: false }
    }
}
```

静的メソッドを呼び出す唯一の方法は、**関数呼び出し構文 (function call syntax)** を使用することです：

```rust
// 関数呼び出し構文: <構造体名>::<メソッド名>(<パラメータ>)
let default_config = Configuration::default();
```

### 等価性

最初のパラメータとして `self` を受け取るメソッドに対しても、関数呼び出し構文を使用することができます：

```rust
// 関数呼び出し構文:
//   <構造体名>::<メソッド名>(<インスタンス>, <パラメータ>)
let is_open = Ticket::is_open(ticket);
```

関数呼び出し構文を使うと、`ticket` がメソッドの最初のパラメータである `self` として使用されていることが非常に明確になりますが、間違いなく冗長になります。可能な限りメソッド呼び出し構文を優先してください。
