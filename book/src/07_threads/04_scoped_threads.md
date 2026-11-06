# スコープ付きスレッド (Scoped threads)

これまで議論してきたすべてのライフタイム問題は、共通の原因を持っています。それは、スポーンされたスレッドが親スレッドよりも長生きする可能性があるということです。\
**スコープ付きスレッド (scoped threads)** を使用することで、この問題を回避できます。

```rust
let v = vec![1, 2, 3];
let midpoint = v.len() / 2;

std::thread::scope(|scope| {
    scope.spawn(|| {
        let first = &v[..midpoint];
        println!("Here's the first half of v: {first:?}");
    });
    scope.spawn(|| {
        let second = &v[midpoint..];
        println!("Here's the second half of v: {second:?}");
    });
});

println!("Here's v: {v:?}");
```

何が起きているのかを紐解いていきましょう。

## `scope`

`std::thread::scope` 関数は、新しい**スコープ (scope)** を作成します。\
`std::thread::scope` は入力として、`Scope` インスタンスという単一の引数を持つクロージャを受け取ります。

## スコープ付きスポーン (Scoped spawns)

`Scope` は `spawn` メソッドを公開しています。\
`std::thread::spawn` とは異なり、`Scope` を使用してスポーンされたすべてのスレッドは、スコープが終了するときに**自動的に `join`（合流・待機）**されます。

もし前の例を `std::thread::spawn` に「翻訳」したとすると、次のようになります：

```rust
let v = vec![1, 2, 3];
let midpoint = v.len() / 2;

let handle1 = std::thread::spawn(|| {
    let first = &v[..midpoint];
    println!("Here's the first half of v: {first:?}");
});
let handle2 = std::thread::spawn(|| {
    let second = &v[midpoint..];
    println!("Here's the second half of v: {second:?}");
});

handle1.join().unwrap();
handle2.join().unwrap();

println!("Here's v: {v:?}");
```

## 環境からの借用

しかし、この翻訳された例はコンパイルできません。`&v` のライフタイムが `'static` ではないため、スポーンされたスレッドからは使用できないとコンパイラが文句を言うからです。

`std::thread::scope` を使えば、これは問題になりません。**周囲の環境から安全に借用する**ことができます。

上記の例では、`v` はスレッドがスポーンされる地点よりも前に作成されています。
そして、`v` がドロップされるのは `scope` が戻った*後*だけです。同時に、`scope` 内でスポーンされたすべてのスレッドは、`scope` が戻る*前*に終了することが保証されているため、ダングリング参照が発生するリスクはありません。

コンパイラも文句を言いません！
