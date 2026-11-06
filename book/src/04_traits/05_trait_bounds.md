# トレイト境界 (Trait bounds)

これまでにトレイトの2つのユースケースを見てきました：

- 「組み込みの」振る舞いを解禁する（例：演算子のオーバーロード）
- 既存の型に新しい振る舞いを追加する（例：拡張トレイト）

そして3つ目のユースケースがあります。それが**ジェネリックプログラミング (generic programming)** です。

## 問題点

これまでのすべての関数とメソッドは、**具体的な型 (concrete types)** を扱ってきました。\
具体的な型を操作するコードは、通常、書くのも理解するのも簡単です。しかし、再利用性には限界があります。\
例えば、整数が偶数であれば `true` を返す関数を書きたいと想像してみてください。
具体的な型を扱う場合、サポートしたい整数型ごとに個別の関数を書かなければなりません：

```rust
fn is_even_i32(n: i32) -> bool {
    n % 2 == 0
}

fn is_even_i64(n: i64) -> bool {
    n % 2 == 0
}

// など
```

あるいは、単一の拡張トレイトを書き、各整数型に対して異なる実装を書くこともできます：

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl IsEven for i64 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

// など
```

いずれにせよ、コードの重複は残ります。

## ジェネリックプログラミング

**ジェネリクス (generics)** を使えば、より上手く書くことができます。\
ジェネリクスを使用すると、具体的な型の代わりに**型パラメータ (type parameter)** を扱うコードを書くことができます：

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
{
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

`print_if_even` は**ジェネリック関数 (generic function)** です。\
特定の入力型に縛られていません。その代わり、以下の条件を満たす任意の型 `T` で動作します：

- `IsEven` トレイトを実装している。
- `Debug` トレイトを実装している。

この契約は**トレイト境界 (trait bound)** によって表現されます：`T: IsEven + Debug`。\
`+` 記号は、`T` が複数のトレイトを実装することを要求するために使用されます。`T: IsEven + Debug` は、「`T` が `IsEven` **かつ** `Debug` を実装している場合」と等価です。

## トレイト境界の役割

`print_if_even` において、トレイト境界はどのような目的を果たしているのでしょうか？\
それを確かめるために、トレイト境界を取り除いてみましょう：

```rust
fn print_if_even<T>(n: T) {
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

このコードはコンパイルできません：

```text
error[E0599]: no method named `is_even` found for type parameter `T` 
              in the current scope
 --> src/lib.rs:2:10
  |
1 | fn print_if_even<T>(n: T) {
  |                  - method `is_even` not found 
  |                    for this type parameter
2 |     if n.is_even() {
  |          ^^^^^^^ method not found in `T`

error[E0277]: `T` doesn't implement `Debug`
 --> src/lib.rs:3:19
  |
3 |         println!("{n:?} is even");
  |                   ^^^^^ 
  |   `T` cannot be formatted using `{:?}` because 
  |         it doesn't implement `Debug`
  |
help: consider restricting type parameter `T`
  |
1 | fn print_if_even<T: std::fmt::Debug>(n: T) {
  |                   +++++++++++++++++
```

トレイト境界がないと、コンパイラは `T` が**何をできるのか**を知ることができません。\
`T` が `is_even` メソッドを持っていることも、出力用に `T` をフォーマットする方法もわからないのです。
コンパイラの視点から見ると、裸の `T` には何の振る舞いもありません。\
トレイト境界は、関数本体が必要とする振る舞いが存在することを保証することで、使用できる型の集合を制限します。

## 構文：トレイト境界のインライン化

上記のすべての例では、トレイト境界を指定するために **`where` 節 (`where` clause)** を使用しました：

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
//  ^^^^^^^^^^^^^^^^^
//  これが `where` 節です
{
    // [...]
}
```

トレイト境界がシンプルな場合は、型パラメータのすぐ横に直接**インライン化 (inline)** することができます：

```rust
fn print_if_even<T: IsEven + Debug>(n: T) {
    //           ^^^^^^^^^^^^^^^^^
    //           これがインラインのトレイト境界です
    // [...]
}
```

## 構文：意味のある名前

上記の例では、型パラメータ名として `T` を使用しました。これは、関数に型パラメータが1つしかない場合の一般的な慣習です。\
しかし、より意味のある名前を使用することを妨げるものは何もありません：

```rust
fn print_if_even<Number: IsEven + Debug>(n: Number) {
    // [...]
}
```

実際、複数の型パラメータが関与する場合や、`T` という名前だけでは関数内での型の役割について十分な情報が伝わらない場合には、意味のある名前を使用することが**望ましい**です。
変数や関数のパラメータと同様に、型パラメータに名前を付ける際も明確さと可読性を最大化しましょう。
ただし、Rustの命名規則には従ってください。[型パラメータ名にはアッパーキャメルケース（UpperCamelCase）を使用します](https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-rfc-430-c-case)。

## 関数シグネチャがすべて

そもそも、なぜトレイト境界が必要なのか疑問に思うかもしれません。コンパイラが関数の本体から必要なトレイトを推論することはできないのでしょうか？\
推論しようと思えばできますが、コンパイラはそれを行いません。\
その理由は、[関数のパラメータに対する明示的な型注釈](../02_basic_calculator/02_variables.md#関数の引数も変数である)と同じです。各関数のシグネチャは呼び出し元と呼び出し先の間の契約であり、その条件は明示的に述べられなければなりません。
これにより、より分かりやすいエラーメッセージ、より優れたドキュメント、バージョン間での意図しない破壊的変更の減少、そしてコンパイル時間の短縮が可能になります。
