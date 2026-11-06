# `Future` トレイト

## ローカルな `Rc` の問題

`tokio::spawn` のシグネチャに戻りましょう：

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
{ /* */ }
```

`F` が `Send` であるとは、*実際には*どういう意味でしょうか？\
前のセクションで見たように、スポーン元の環境からキャプチャするあらゆる値が `Send` でなければならないことを意味します。しかし、それだけにとどまりません。

*.await ポイントをまたいで保持される*あらゆる値も `Send` でなければならないのです。\
例を見てみましょう：

```rust
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    // 非同期関数の*内部*で作成された、
    // `Send` ではない値
    let non_send = Rc::new(1);
    
    // 何もしない `.await` ポイント
    yield_now().await;

    // `.await` の後でも、ローカルな非 `Send` の値が
    // 引き続き必要とされている
    println!("{}", non_send);
}
```

コンパイラはこのコードを拒否します：

```text
error: future cannot be sent between threads safely
    |
5   |     tokio::spawn(example());
    |                  ^^^^^^^^^ 
    | future returned by `example` is not `Send`
    |
note: future is not `Send` as this value is used across an await
    |
11  |     let non_send = Rc::new(1);
    |         -------- has type `Rc<i32>` which is not `Send`
12  |     // A `.await` point
13  |     yield_now().await;
    |                 ^^^^^ 
    |   await occurs here, with `non_send` maybe used later
note: required by a bound in `tokio::spawn`
    |
164 |     pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    |            ----- required by a bound in this function
165 |     where
166 |         F: Future + Send + 'static,
    |                     ^^^^ required by this bound in `spawn`
```

なぜこうなるのかを理解するには、Rustの非同期モデルについての理解をさらに深める必要があります。

## `Future` トレイト

最初の頃、`async` 関数は `Future` トレイトを実装する型である**フューチャー (futures)** を返すと述べました。フューチャーは**ステートマシン（状態機械）**だと考えることができます。
フューチャーは以下の2つの状態のいずれかにあります：

- **pending（保留中）**：計算がまだ終了していない。
- **ready（完了）**：計算が終了し、ここに出力がある。

これはトレイト定義の中で次のように表現されています：

```rust
trait Future {
    type Output;
    
    // 今のところは `Pin` と `Context` を無視してください
    fn poll(
      self: Pin<&mut Self>, 
      cx: &mut Context<'_>
    ) -> Poll<Self::Output>;
}
```

### `poll`

`poll` メソッドは `Future` トレイトの心臓部です。\
フューチャーはそれ単体では何もしません。処理を進めるには**ポーリング (poll)** される必要があります。\
`poll` を呼び出すことは、フューチャーに作業を進めるよう依頼することです。
`poll` は処理を進めようと試み、以下のいずれかを返します：

- `Poll::Pending`：フューチャーはまだ完了していません。後でもう一度 `poll` を呼び出す必要があります。
- `Poll::Ready(value)`：フューチャーが完了しました。`value` は計算結果であり、その型は `Self::Output` です。

`Future::poll` が一度 `Poll::Ready` を返したら、それを再度ポーリングしてはなりません。フューチャーは完了しており、やるべきことは何も残っていないからです。

### ランタイムの役割

開発者が直接 `poll` を呼び出すことは、あったとしてもごく稀です。\
それは非同期ランタイムの仕事です。ランタイムは、フューチャーが進められるときにいつでも確実に処理を進められるようにするために必要なすべての情報（`poll` のシグネチャにある `Context`）を持っています。

## `async fn` とフューチャー

これまで高レベルのインターフェースである非同期関数を扱ってきました。\
そして今、低レベルのプリミティブである `Future` トレイトを見ました。

これら2つはどのように関連しているのでしょうか？

関数を非同期（`async`）としてマークするたびに、その関数はフューチャーを返すようになります。
コンパイラは、非同期関数の本体を**ステートマシン**に変換します。`.await` ポイントごとに1つの状態が作られます。

先ほどの `Rc` の例に戻りましょう：

```rust
use std::rc::Rc;
use tokio::task::yield_now;

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
```

コンパイラはこれを、おおよそ次のような列挙型に変換します：

```rust
pub enum ExampleFuture {
    NotStarted,
    YieldNow(Rc<i32>),
    Terminated,
}
```

`example` が呼び出されると、`ExampleFuture::NotStarted` を返します。このフューチャーはまだ一度もポーリングされていないため、何も起きていません。\
ランタイムが最初にそれをポーリングすると、`ExampleFuture` は次の `.await` ポイントまで処理を進め、ステートマシンの `ExampleFuture::YieldNow(Rc<i32>)` の段階で停止し、`Poll::Pending` を返します。\
再度ポーリングされると、残りのコード（`println!`）を実行し、`Poll::Ready(())` を返します。

ステートマシン表現である `ExampleFuture` を見れば、なぜ `example` が `Send` ではないのかが明確になります。内部に `Rc` を保持しているため、`Send` になり得ないのです。

## イールドポイント (Yield points)

`example` で今見たように、すべての `.await` ポイントはフューチャーのライフサイクルにおいて新しい中間状態を作り出します。\
そのため、`.await` ポイントは**イールドポイント (yield points / 譲渡地点)** とも呼ばれます。フューチャーは自身をポーリングしていたランタイムに*制御を譲り渡し (yield)*、ランタイムがそのフューチャーを一時停止して（必要であれば）別のタスクの実行をスケジュールできるようにします。これにより、複数の処理を並行して進めることができるのです。

制御を譲ること（yielding）の重要性については、後のセクションで再び取り上げます。
