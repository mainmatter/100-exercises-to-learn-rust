# タスクのスポーン (Spawning tasks)

前回の演習の解答は、おそらく次のようになっているはずです：

```rust
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        let (mut reader, mut writer) = socket.split();
        tokio::io::copy(&mut reader, &mut writer).await?;
    }
}
```

これは悪くありません！\
2つの着信接続の間に長い時間が経過した場合、`echo` 関数はアイドル状態になり（`TcpListener::accept` は非同期関数であるため）、エグゼキュータはその間に他のタスクを実行できます。

しかし、どうすれば実際に複数のタスクを並行して実行できるのでしょうか？\
もし非同期関数を常に（`.await` を使って）完了するまで実行していたら、一度に実行されるタスクは決して1つ以上にはなりません。

そこで登場するのが `tokio::spawn` 関数です。

## `tokio::spawn`

`tokio::spawn` を使用すると、**完了を待つことなく**タスクをエグゼキュータに引き渡すことができます。\
`tokio::spawn` を呼び出すたびに、スポーンした元のタスクと**並行して**、スポーンされたタスクをバックグラウンドで実行し続けるよう `tokio` に指示していることになります。

これを使用して複数の接続を並行して処理する方法は次の通りです：

```rust
use tokio::net::TcpListener;

pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        // 接続を処理するためのバックグラウンドタスクをスポーンし、
        // メインタスクが即座に新しい接続の受け入れを再開できるようにします
        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();
            tokio::io::copy(&mut reader, &mut writer).await?;
        });
    }
}
```

### 非同期ブロック (Asynchronous blocks)

この例では、`tokio::spawn` に**非同期ブロック (asynchronous block)** である `async move { /* */ }` を渡しています。
非同期ブロックは、個別の非同期関数を定義することなく、コードの特定の領域を非同期としてマークするための手軽な方法です。

### `JoinHandle`

`tokio::spawn` は `JoinHandle` を返します。\
スポーンされたスレッドに対して `join` を使用したのと同じように、`JoinHandle` を使用してバックグラウンドタスクの完了を `.await`（待機）することができます。

```rust
pub async fn run() {
    // リモートサーバーにテレメトリデータを送信するための
    // バックグラウンドタスクをスポーンする
    let handle = tokio::spawn(emit_telemetry());
    // その間に、他の有用な作業を行う
    do_work().await;
    // ただし、テレメトリデータが正常に配信されるまでは
    // 呼び出し元に戻らないようにする
    handle.await;
}

pub async fn emit_telemetry() {
    // [...]
}

pub async fn do_work() {
    // [...]
}
```

### パニック境界 (Panic boundary)

`tokio::spawn` でスポーンされたタスクがパニックを起こした場合、そのパニックはエグゼキュータによって捕捉（キャッチ）されます。\
対応する `JoinHandle` を `.await` しない場合、パニックはスポーン元には伝播しません。
また、`JoinHandle` を `.await` したとしても、パニックが自動的に伝播するわけではありません。
`JoinHandle` を `.await` すると、エラー型として [`JoinError`](https://docs.rs/tokio/latest/tokio/task/struct.JoinError.html) を持つ `Result` が返されます。その後、`JoinError::is_panic` を呼び出すことでタスクがパニックしたかどうかを確認でき、そのパニックをどう処理するか（ログに記録する、無視する、または伝播させる）を選択できます。

```rust
use tokio::task::JoinError;

pub async fn run() {
    let handle = tokio::spawn(work());
    if let Err(e) = handle.await {
        if let Ok(reason) = e.try_into_panic() {
            // タスクがパニックを起こしました。
            // パニックの巻き戻し（アンワインド）を再開し、
            // 現在のタスクへとパニックを伝播させます。
            panic::resume_unwind(reason);
        }
    }
}

pub async fn work() {
    // [...]
}
```

### `std::thread::spawn` と `tokio::spawn` の違い

`tokio::spawn` は、`std::thread::spawn` の非同期版の兄弟だと考えることができます。

ただし、重要な違いに注目してください。`std::thread::spawn` では、制御をOSのスケジューラに委譲しています。
スレッドがどのようにスケジュールされるかを開発者が制御することはできません。

一方、`tokio::spawn` では、完全にユーザー空間（ユーザースペース）で動作する非同期エグゼキュータに委譲しています。次にどのタスクを実行するかの決定に、基盤となるOSのスケジューラは関与しません。使用するエグゼキュータを通じて、私たち自身がその決定を担うことになるのです。
