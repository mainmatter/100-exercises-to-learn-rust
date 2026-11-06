use tokio::net::TcpListener;

// TODO: 2つのリスナーで並行してTCP接続を受け入れるエコーサーバーを書いてください。
//  （同じリスナー上の）複数の接続も並行して処理される必要があります。
//  受信したデータはクライアントにそのままエコーバック（送り返す）される必要があります。
pub async fn echoes(first: TcpListener, second: TcpListener) -> Result<(), anyhow::Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        tokio::spawn(echoes(first_listener, second_listener));

        let requests = vec!["hello", "world", "foo", "bar"];
        let mut join_set = JoinSet::new();

        for request in requests.clone() {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, mut writer) = socket.split();

                    // リクエストを送信する
                    writer.write_all(request.as_bytes()).await.unwrap();
                    // ソケットの書き込み側を閉じる
                    writer.shutdown().await.unwrap();

                    // レスポンスを読み取る
                    let mut buf = Vec::with_capacity(request.len());
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, request.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
