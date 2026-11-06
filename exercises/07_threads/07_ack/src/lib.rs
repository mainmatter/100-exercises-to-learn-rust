use std::sync::mpsc::{Receiver, Sender};
use crate::store::TicketStore;

pub mod data;
pub mod store;

// 期待されるスキーマを理解するために、テストを参照してください。
pub enum Command {
    Insert { todo!() },
    Get { todo!() }
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: 期待通りに受信したコマンドを処理してください。
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {}) => {
                todo!()
            }
            Ok(Command::Get {
                todo!()
            }) => {
                todo!()
            }
            Err(_) => {
                // 送信側（Sender）がもう存在しないため、安全に break して
                // サーバーをシャットダウンできます。
                break
            },
        }
    }
}
