# スレッド (Threads)

マルチスレッドのコードを書く前に、一歩下がってスレッドとは何か、そしてなぜスレッドを使いたいのかについてお話ししましょう。

## スレッドとは何か？

**スレッド (thread)** とは、基盤となるオペレーティングシステム（OS）によって管理される実行コンテキストのことです。\
各スレッドは、独自のスタックと命令ポインタを持っています。

単一の**プロセス (process)** は複数のスレッドを管理できます。
これらのスレッドは同じメモリ空間を共有しており、つまり同じデータにアクセスできます。

スレッドは**論理的な**構成要素です。結局のところ、**物理的な**実行ユニットであるCPUコア上では、一度に1つの命令セットしか実行できません。\
CPUコアの数よりもはるかに多くのスレッドが存在し得るため、OSの**スケジューラ (scheduler)** が、スループットと応答性を最大化するようにCPU時間をスレッド間で分割し、どの時点でどのスレッドを実行するかを決定する役割を担います。

## `main`

Rustプログラムが起動すると、単一のスレッド、すなわち**メインスレッド (main thread)** 上で実行されます。\
このスレッドはOSによって作成され、`main` 関数を実行する責任を負います。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}
```

## `std::thread`

Rustの標準ライブラリは、スレッドを作成および管理するための `std::thread` モジュールを提供しています。

### `spawn`

`std::thread::spawn` を使用して新しいスレッドを作成（スポーン）し、その上でコードを実行できます。

例えば：

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("Hello from a thread!");
        }
    });
    
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}
```

このプログラムを [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=afedf7062298ca8f5a248bc551062eaa) で実行すると、メインスレッドとスポーンされたスレッドが並行して実行されることがわかります。\
各スレッドは互いに独立して処理を進めます。

### プロセスの終了

メインスレッドが終了すると、プロセス全体が終了します。\
スポーンされたスレッドは、自身の処理が終了するか、メインスレッドが終了するまで実行を続けます。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("Hello from a thread!");
        }
    });

    thread::sleep(Duration::from_secs(5));
}
```

上記の例では、"Hello from a thread!" というメッセージがおよそ5回出力されることが期待できます。\
その後、メインスレッドが終了し（`sleep` の呼び出しから戻ったとき）、プロセス全体が終了するため、スポーンされたスレッドも強制終了されます。

### `join`

`spawn` が返す `JoinHandle` に対して `join` メソッドを呼び出すことで、スポーンされたスレッドが終了するのを待つこともできます。

```rust
use std::thread;
fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}
```

この例では、メインスレッドは終了する前に、スポーンされたスレッドが終了するのを待機します。\
これにより、2つのスレッド間に一種の**同期 (synchronization)** が導入されます。スポーンされたスレッドが終了するまでメインスレッドは終了しないため、プログラムが終了する前に "Hello from a thread!" というメッセージが出力されることが保証されます。
