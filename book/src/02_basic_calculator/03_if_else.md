# 制御フロー (パート1)

これまでのプログラムはどれも非常に単純なものでした。\
一連の命令が上から下へと順番に実行され、それだけでした。

ここで**条件分岐 (branching)** を導入しましょう。

## `if` 節

`if` キーワードは、条件が真 (`true`) の場合にのみコードブロックを実行するために使用されます。

簡単な例を見てみましょう：

```rust
let number = 3;
if number < 5 {
    println!("`number` is smaller than 5");
}
```

条件 `number < 5` が真であるため、このプログラムは `` `number` is smaller than 5 `` と出力します。

### `else` 節

ほとんどのプログラミング言語と同様に、Rustも `if` 式の条件が偽 (`false`) のときにコードブロックを実行するためのオプションの `else` 分岐をサポートしています。\
例えば：

```rust
let number = 3;

if number < 5 {
    println!("`number` is smaller than 5");
} else {
    println!("`number` is greater than or equal to 5");
}
```

### `else if` 節

複数の `if` 式が入れ子（ネスト）になっていると、コードがどんどん右側にずれていってしまいます。

```rust
let number = 3;

if number < 5 {
    println!("`number` is smaller than 5");
} else {
    if number < 8 {
        println!("`number` is greater than or equal to 5, but smaller than 8");
    } else {
        println!("`number` is greater than or equal to 8");
    }
}
```

`else if` キーワードを使用すると、複数の `if` 式を1つにまとめることができます：

```rust
let number = 3;

if number < 5 {
    println!("`number` is smaller than 5");
} else if number < 8 {
    println!("`number` is greater than or equal to 5, but smaller than 8");
} else {
    println!("`number` is greater than or equal to 8");
}
```

## 真偽値 (Booleans)

`if` 式の条件は、**真偽値 (boolean)** である `bool` 型でなければなりません。\
真偽値は、整数と同様にRustのプリミティブ型です。

真偽値は、`true`（真）または `false`（偽）の2つの値のいずれかをとります。

### truthy や falsy な値は存在しない

`if` 式の条件が真偽値でない場合、コンパイルエラーが発生します。

例えば、以下のコードはコンパイルできません：

```rust
let number = 3;
if number {
    println!("`number` is not zero");
}
```

以下のコンパイルエラーが得られます：

```text
error[E0308]: mismatched types
 --> src/main.rs:3:8
  |
3 |     if number {
  |        ^^^^^^ expected `bool`, found integer
```

これは型強制に関するRustの哲学に従ったものです。真偽値以外の型から真偽値への自動変換は行われません。
Rustには、JavaScriptやPythonのような **truthy（真とみなされる値）** や **falsy（偽とみなされる値）** という概念はありません。\
チェックしたい条件は明示的に記述する必要があります。

### 比較演算子

`if` 式の条件を組み立てるために比較演算子を使うのは非常に一般的です。\
Rustで整数を扱う際に使用できる比較演算子は以下の通りです：

- `==`：等しい
- `!=`：等しくない
- `<`：より小さい（未満）
- `>`：より大きい
- `<=`：以下
- `>=`：以上

## `if/else` は式 (expression) である

Rustでは、`if` 式は文 (statement) ではなく**式 (expression)** です。つまり、値を返します。\
その値は変数に代入したり、他の式の中で使用したりすることができます。例えば：

```rust
let number = 3;
let message = if number < 5 {
    "smaller than 5"
} else {
    "greater than or equal to 5"
};
```

上記の例では、`if` の各分岐が文字列リテラルに評価され、それが `message` 変数に代入されます。\
唯一の要件は、`if` の両方の分岐が同じ型を返すことです。
