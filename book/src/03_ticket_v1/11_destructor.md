# デストラクタ (Destructors)

ヒープを紹介した際、割り当てたメモリを解放する責任は開発者にあると述べました。\
また、ボローチェッカーを紹介した際には、Rustではメモリを直接管理しなければならないことは滅多にないとも述べました。

これら2つの記述は、一見すると矛盾しているように思えるかもしれません。
**スコープ (scopes)** と**デストラクタ (destructors)** を導入することで、これらがどのように噛み合っているのかを見ていきましょう。

## スコープ (Scopes)

変数の**スコープ**とは、その変数が有効である（または**生きている (alive)**）Rustコードの領域のことです。

変数のスコープは、その宣言から始まります。
そして、次のいずれかが発生したときに終了します：

1. 変数が宣言されたブロック（つまり `{}` の間のコード）が終了したとき
   ```rust
   fn main() {
      // `x` はここではまだスコープに入っていません
      let y = "Hello".to_string();
      let x = "World".to_string(); // <-- x のスコープはここから始まり...
      let h = "!".to_string(); //   |
   } //  <-------------- ...ここで終わります
   ```
2. 変数の所有権が他の誰か（関数や別の変数など）に譲渡（ムーブ）されたとき
   ```rust
   fn compute(t: String) {
      // 何か処理をする [...]
   }

   fn main() {
       let s = "Hello".to_string(); // <-- s のスコープはここから始まり...
                   //                    | 
       compute(s); // <------------------- ..ここで終わります
                   //   `s` が `compute` にムーブされるためです
   }
   ```

## デストラクタ (Destructors)

値の所有者がスコープを抜けると、Rustはその**デストラクタ (destructor)** を呼び出します。\
デストラクタは、その値によって使用されていたリソース（特に、割り当てられたあらゆるメモリ）をクリーンアップしようとします。

値を `std::mem::drop` に渡すことで、その値のデストラクタを手動で呼び出すこともできます。\
そのため、Rust開発者が「値がスコープを抜けてデストラクタが呼び出された」という意味で、「その値は**ドロップされた (dropped)**」と表現するのをよく耳にするでしょう。

### ドロップ位置の可視化

コンパイラが私たちのために行ってくれることを「明示する」ために、`drop` の明示的な呼び出しを挿入してみることができます。先ほどの例に戻りましょう：

```rust
fn main() {
   let y = "Hello".to_string();
   let x = "World".to_string();
   let h = "!".to_string();
}
```

これは以下と等価です：

```rust
fn main() {
   let y = "Hello".to_string();
   let x = "World".to_string();
   let h = "!".to_string();
   // 変数は宣言された順序と逆の順序でドロップされます
   drop(h);
   drop(x);
   drop(y);
}
```

次は、`s` の所有権が `compute` に譲渡される2つ目の例を見てみましょう：

```rust
fn compute(s: String) {
   // 何か処理をする [...]
}

fn main() {
   let s = "Hello".to_string();
   compute(s);
}
```

これは以下と等価です：

```rust
fn compute(t: String) {
    // 何か処理をする [...]
    drop(t); // <-- この時点より前に `t` がドロップされたりムーブされたり
             //     していないと仮定すると、スコープを抜けるここで
             //     コンパイラが `drop` を呼び出します
}

fn main() {
    let s = "Hello".to_string();
    compute(s);
}
```

違いに注目してください。`main` 内で `compute` が呼び出された後、`s` はもはや有効ではありませんが、`main` 内には `drop(s)` がありません。
値の所有権を関数に譲渡すると、**それをクリーンアップする責任も譲渡する**ことになるのです。

これにより、ある値に対するデストラクタが**高々[^leak]1回だけ**呼び出されることが保証され、設計上[二重解放バグ (double free bugs)](https://owasp.org/www-community/vulnerabilities/Doubly_freeing_memory) が防止されます。

### ドロップ後の使用 (Use after drop)

値がドロップされた後にそれを使用しようとするとどうなるでしょうか？

```rust
let x = "Hello".to_string();
drop(x);
println!("{}", x);
```

このコードをコンパイルしようとすると、エラーが発生します：

```rust
error[E0382]: use of moved value: `x`
 --> src/main.rs:4:20
  |
3 |     drop(x);
  |          - value moved here
4 |     println!("{}", x);
  |                    ^ value used here after move
```

`drop` は呼び出された対象の値を**消費**します。つまり、呼び出しの後、その値はもはや有効ではありません。\
したがって、コンパイラはそれを使用することを防ぎ、[解放後使用バグ (use-after-free bugs)](https://owasp.org/www-community/vulnerabilities/Using_freed_memory) を回避します。

### 参照のドロップ

変数が参照を保持している場合はどうなるでしょうか？\
例えば：

```rust
let x = 42i32;
let y = &x;
drop(y);
```

`drop(y)` を呼び出しても……何も起こりません。\
実際にこのコードをコンパイルしようとすると、警告が表示されます：

```text
warning: calls to `std::mem::drop` with a reference 
         instead of an owned value does nothing
 --> src/main.rs:4:5
  |
4 |     drop(y);
  |     ^^^^^-^
  |          |
  |          argument has type `&i32`
  |
```

これは先ほど述べたことに立ち返ります。デストラクタは1回だけ呼び出したいのです。\
同じ値に対して複数の参照を持つことができます。もし、そのうちの1つがスコープを抜けたときに、指し示している値のデストラクタを呼び出してしまったら、残りの参照はどうなってしまうでしょうか？
それらはもはや有効ではないメモリ位置を参照することになります。いわゆる[**ダングリングポインタ (dangling pointer)**](https://en.wikipedia.org/wiki/Dangling_pointer) であり、[**解放後使用バグ (use-after-free bugs)**](https://owasp.org/www-community/vulnerabilities/Using_freed_memory) の近親者です。
Rustの所有権システムは、設計上このような種類のバグを排除しています。

[^leak]: Rustはデストラクタが必ず実行されることを保証しているわけではありません。例えば、明示的に[メモリをリークさせる](../07_threads/03_leak.md)ことを選択した場合などには実行されません。
