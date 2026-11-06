use tokio::net::TcpListener;

// TODO: 着信TCP接続を受け入れ、受信したデータをクライアントにそのまま送り返す（エコーする）
//  エコーサーバーを書いてください。
//  `echo` は1つの接続の処理を終えてもリターンせず、新しい接続を受け入れ続ける必要があります。
//
// ヒント：エコーサーバーを実装するには、`tokio` の構造体とメソッドに頼る必要があります。
// 具体的には：
// - 次の着信接続を処理するための `tokio::net::TcpListener::accept`
// - ソケットからリーダー（読み取り側）とライター（書き込み側）を取得するための `tokio::net::TcpStream::split`
// - リーダーからライターへデータをコピーするための `tokio::io::copy`
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_echo() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(echo(listener));

        let requests = vec!["hello", "world", "foo", "bar"];

        for request in requests {
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
        }
    }
}
