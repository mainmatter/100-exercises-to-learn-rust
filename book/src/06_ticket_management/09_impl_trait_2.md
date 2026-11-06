# 引数位置の `impl Trait`

前のセクションでは、型の名前を指定せずに型を返すために `impl Trait` を使用する方法を見ました。\
同じ構文を**引数の位置 (argument position)** でも使用できます：

```rust
fn print_iter(iter: impl Iterator<Item = i32>) {
    for i in iter {
        println!("{}", i);
    }
}
```

`print_iter` は `i32` のイテレータを受け取り、各要素を出力します。\
**引数の位置**で使用された場合、`impl Trait` はトレイト境界を持つジェネリックパラメータと等価です：

```rust
fn print_iter<T>(iter: T) 
where
    T: Iterator<Item = i32>
{
    for i in iter {
        println!("{}", i);
    }
}
```

## 欠点

経験則として、引数の位置では `impl Trait` よりもジェネリクスを優先してください。\
ジェネリクスを使用すると、呼び出し元はターボフィッシュ構文 (`::<>`) を使って引数の型を明示的に指定できるため、型の曖昧さを解消する際に役立ちます。`impl Trait` の場合はそれができません。
