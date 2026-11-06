# キャンセル (Cancellation)

保留中（pending）のフューチャーがドロップされるとどうなるでしょうか？\
ランタイムはもはやそれをポーリングしなくなるため、それ以上処理が進むことはありません。
言い換えれば、その実行は**キャンセル (cancelled)** されたことになります。

実際の開発では、タイムアウトを扱う際によくこの状況が発生します。
例えば：

```rust
use tokio::time::timeout;
use tokio::sync::oneshot;
use std::time::Duration;

async fn http_call() {
    // [...]
}

async fn run() {
    // 10ミリ秒で期限切れになる `Timeout` でフューチャーをラップします。
    let duration = Duration::from_millis(10);
    if let Err(_) = timeout(duration, http_call()).await {
        println!("Didn't receive a value within 10 ms");
    }
}
```

タイムアウトが期限切れになると、`http_call` によって返されたフューチャーはキャンセルされます。
`http_call` の本体が次のようになっていると想像してみましょう：

```rust
use std::net::TcpStream;

async fn http_call() {
    let (stream, _) = TcpStream::connect(/* */).await.unwrap();
    let request: Vec<u8> = /* */;
    stream.write_all(&request).await.unwrap();
}
```

各イールドポイントが**キャンセルポイント (cancellation point)** になります。\
`http_call` はランタイムによってプリエンプト（強制中断）できないため、`.await` を介してエグゼキュータに制御を譲り渡した後にしか破棄できません。
これは再帰的に適用されます。例えば、`stream.write_all(&request)` の実装内部には複数のイールドポイントが存在する可能性が高いです。したがって、`http_call` がキャンセルされる前に*部分的な*リクエストを送信してしまい、接続がドロップされてボディの送信が完了しないまま終わる、といったことが十分に起こり得ます。

## クリーンアップ (Clean up)

Rustのキャンセル機構は非常に強力です。タスク側からのいかなる協力も必要とせずに、呼び出し元が進行中のタスクをキャンセルできるからです。\
しかし同時に、これは非常に危険でもあります。操作を中断する前にいくつかのクリーンアップ作業が確実に実行されるよう、**グレースフルな（段階的で安全な）キャンセル (graceful cancellation)** を行いたい場合があるでしょう。

例えば、SQLトランザクションのための以下のような架空のAPIを考えてみてください：

```rust
async fn transfer_money(
    connection: SqlConnection,
    payer_id: u64,
    payee_id: u64,
    amount: u64
) -> Result<(), anyhow::Error> {
    let transaction = connection.begin_transaction().await?;
    update_balance(payer_id, amount, &transaction).await?;
    decrease_balance(payee_id, amount, &transaction).await?;
    transaction.commit().await?;
}
```

キャンセル時には、保留中のトランザクションを宙ぶらりんのまま放置するのではなく、明示的にアボート（ロールバック）するのが理想的です。
しかし残念ながら、Rustはこの種の**非同期な**クリーンアップ操作に対する完璧な（絶対確実な）仕組みを提供していません。

最も一般的な戦略は、`Drop` トレイトに頼って必要なクリーンアップ作業をスケジュールすることです。これには以下のような方法があります：

- ランタイム上に新しいタスクをスポーンする
- チャネルにメッセージをキューイングする
- バックグラウンドスレッドをスポーンする

どれが最適な選択かは文脈によって異なります。

## スポーンされたタスクのキャンセル

`tokio::spawn` を使用してタスクをスポーンすると、そのタスクはランタイムの所有物になるため、呼び出し元で直接ドロップすることはできなくなります。\
それでも、必要であれば `JoinHandle` を使用してキャンセルを要求することができます：

```rust
async fn run() {
    let handle = tokio::spawn(/* 何らかの非同期タスク */);
    // スポーンされたタスクをキャンセルする
    handle.abort();
}
```

## さらに詳しく知りたい方へ

- 2つの異なるフューチャーを「競争（race）」させるために `tokio` の `select!` マクロを使用する際は、細心の注意を払ってください。**キャンセル安全性 (cancellation safety)** を保証できない限り、ループ内で同じタスクをリトライするのは危険です。詳細については [`select!` のドキュメント](https://tokio.rs/tokio/tutorial/select)を確認してください。\
  2つの非同期データストリーム（ソケットとチャネルなど）をインターリーブ（合流）させる必要がある場合は、代わりに [`StreamExt::merge`](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.merge) を使用することを優先してください。
- ケースによっては、`JoinHandle::abort` よりも [`CancellationToken`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html) の方が望ましい場合があります。
