# `'static` ライフタイム

前回の演習でベクタからスライスを借用しようとした場合、おそらく次のようなコンパイルエラーに遭遇したことでしょう：

```text
error[E0597]: `v` does not live long enough
   |
11 | pub fn sum(v: Vec<i32>) -> i32 {
   |            - binding `v` declared here
...
15 |     let right = &v[split_point..];
   |                  ^ borrowed value does not live long enough
16 |     let left_handle = spawn(move || left.iter().sum::<i32>());
   |                             --------------------------------
                     argument requires that `v` is borrowed for `'static`
19 | }
   |  - `v` dropped here while still borrowed
```

`argument requires that v is borrowed for 'static`（引数は `v` が `'static` の期間借用されることを要求しています）とは、どういう意味でしょうか？

`'static` ライフタイムは、Rustにおける特別なライフタイムです。\
これは、その値がプログラムの実行期間全体にわたって有効であることを意味します。

## デタッチされたスレッド (Detached threads)

`thread::spawn` を介して起動されたスレッドは、それをスポーンした親スレッドよりも**長生きする (outlive)** ことができます。\
例えば：

```rust
use std::thread;

fn f() {
    thread::spawn(|| {
        thread::spawn(|| {
            loop {
                thread::sleep(std::time::Duration::from_secs(1));
                println!("Hello from the detached thread!");
            }
        });
    });
}
```

この例では、最初にスポーンされたスレッドが、1秒ごとにメッセージを出力する子スレッドをさらにスポーンします。\
その後、最初のスレッドは処理を完了して終了します。その際、子スレッドはプロセス全体が実行されている限り**実行を継続**します。\
Rustの用語では、子スレッドが親スレッドよりも**長生きした (outlived)** と言います。

## `'static` ライフタイムの要件

スポーンされたスレッドは以下の可能性があるため：

- それをスポーンしたスレッド（親スレッド）よりも長生きする
- プログラムが終了するまで実行され続ける

プログラムが終了する前にドロップされる可能性のあるいかなる値も借用してはなりません。この制約に違反すると、解放後使用（use-after-free）バグの危険にさらされることになります。\
`std::thread::spawn` のシグネチャが、渡されるクロージャに対して `'static` ライフタイムを要求しているのはそのためです：

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static
{
    // [..]
}
```

## `'static` は参照（だけ）に関するものではない

Rustのすべての値にはライフタイムがあり、それは参照だけに限った話ではありません。

特に、自身のデータを所有する型（`Vec` や `String` など）は `'static` 制約を満たします。それを所有している場合、最初にそれを作成した関数が戻った後であっても、必要な限りずっとそれを使い続けることができるからです。

したがって、`'static` は次のように要求する方法だと解釈できます：

- 所有された（ownedな）値を渡すか、
- プログラムの実行期間全体にわたって有効な参照を渡すこと。

1つ目のアプローチが、前回の演習で問題を解決した方法です。つまり、元のベクタの左半分と右半分を保持するために新しいベクタを割り当て、それらをスポーンされたスレッドにムーブしました。

## `'static` 参照

では、2つ目のケース、つまりプログラムの実行期間全体にわたって有効な参照について話しましょう。

### 静的データ (Static data)

最も一般的なケースは、文字列リテラルのような**静的データ (static data)** への参照です：

```rust
let s: &'static str = "Hello world!";
```

文字列リテラルはコンパイル時にわかっているため、Rustはそれらを実行可能ファイルの*内部*にある**読み取り専用データセグメント (read-only data segment)** と呼ばれる領域に保存します。
したがって、その領域を指すすべての参照は、プログラムが実行されている限りずっと有効であり、`'static` の契約を満たします。

## さらに詳しく知りたい方へ

- [データセグメント (The data segment)](https://en.wikipedia.org/wiki/Data_segment)
