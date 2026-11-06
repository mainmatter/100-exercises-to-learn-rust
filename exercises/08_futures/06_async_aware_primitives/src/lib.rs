/// TODO: 以下のコードは、非同期対応ではない `std` のチャネルを使用しているためデッドロックします。
///  `tokio` のチャネルプリミティブを使用するように書き直してください
///  （はい、テストコードにも手を加える必要があります）。
///
/// デッドロックにつながる一連のイベントの流れを理解できますか？
use std::sync::mpsc;

pub struct Message {
    payload: String,
    response_channel: mpsc::Sender<Message>,
}

/// 受信した任意のメッセージに対して `pong` で応答し、呼び出し元との通信を
/// 継続するために新しいチャネルをセットアップします。
pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        if let Ok(msg) = receiver.recv() {
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = mpsc::channel();
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .unwrap();
            receiver = new_receiver;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    use std::sync::mpsc;

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = mpsc::channel();
        let (response_sender, response_receiver) = mpsc::channel();
        sender
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender,
            })
            .unwrap();

        tokio::spawn(pong(receiver));

        let answer = response_receiver.recv().unwrap().payload;
        assert_eq!(answer, "pong");
    }
}
