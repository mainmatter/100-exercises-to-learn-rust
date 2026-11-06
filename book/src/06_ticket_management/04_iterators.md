# イテレーション (反復処理)

ごく初期の演習の中で、Rustでは `for` ループを使ってコレクションを反復処理（イテレート）できることを学びました。
その時点では範囲（例：`0..5`）を見ていましたが、配列やベクタのようなコレクションに対しても同じことが言えます。

```rust
// `Vec` に対して機能します
let v = vec![1, 2, 3];
for n in v {
    println!("{}", n);
}

// 配列に対しても機能します
let a: [u32; 3] = [1, 2, 3];
for n in a {
    println!("{}", n);
}
```

これが内部でどのように機能しているのかを理解する時が来ました。

## `for` の脱糖 (Desugaring)

Rustで `for` ループを書くたびに、コンパイラはそれを以下のコードに*脱糖 (desugar / 糖衣構文を展開)*します：

```rust
let mut iter = IntoIterator::into_iter(v);
loop {
    match iter.next() {
        Some(n) => {
            println!("{}", n);
        }
        None => break,
    }
}
```

`loop` は、`for` や `while` に加わるもう1つのループ構造です。\
`loop` ブロックは、明示的に `break` で抜け出さない限り、永久に実行され続けます。

## `Iterator` トレイト

先ほどのコードスニペットの `next` メソッドは、`Iterator` トレイトに由来するものです。
`Iterator` トレイトはRustの標準ライブラリで定義されており、一連の値を生成できる型に対して共通のインターフェースを提供します：

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

関連型 `Item` は、イテレータによって生成される値の型を指定します。

`next` はシーケンス内の次の値を返します。\
返す値がある場合は `Some(value)` を返し、ない場合は `None` を返します。

注意してください。イテレータが `None` を返したからといって、そのイテレータが枯渇した（以降もずっと `None` を返し続ける）という保証はありません。それが保証されるのは、イテレータが（より制約の強い）[`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html) トレイトを実装している場合のみです。

## `IntoIterator` トレイト

すべての型が `Iterator` を実装しているわけではありませんが、多くの型は `Iterator` を実装する型に変換できます。\
そこで登場するのが `IntoIterator` トレイトです：

```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}
```

`into_iter` メソッドは元の値を消費し、その要素に対するイテレータを返します。\
ある型が持つことができる `IntoIterator` の実装はただ1つだけです。`for` が何に脱糖されるべきかについて曖昧さがあってはならないからです。

細かい点ですが、`Iterator` を実装するすべての型は、自動的に `IntoIterator` も実装します。
`into_iter` から単に自分自身を返すだけです！

## 境界チェック (Bounds checks)

イテレータを使った反復処理には嬉しい副作用があります。設計上、境界外（範囲外）に出ることがあり得ないのです。\
これにより、Rustは生成される機械語コードから境界チェックを取り除くことができ、反復処理がより高速になります。

つまり、

```rust
let v = vec![1, 2, 3];
for n in v {
    println!("{}", n);
}
```

は通常、次のコードよりも高速です：

```rust
let v = vec![1, 2, 3];
for i in 0..v.len() {
    println!("{}", v[i]);
}
```

このルールには例外もあります。手動でインデックスアクセスを行っていても、境界外に出ないことをコンパイラが証明できる場合があり、その場合はいずれにせよ境界チェックが削除されます。しかし一般的には、可能な限りインデックスアクセスよりもイテレータによる反復処理を優先してください。
