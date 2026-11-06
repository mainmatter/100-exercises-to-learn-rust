use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    Insert(todo!()),
}

// サーバースレッドをスポーンしてシステムを起動します。
// 1つ以上のクライアントがサーバーとやり取りするために使用できる
// `Sender` インスタンスを返します。
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: サーバータスクは**決して**停止してはいけません。
//  ループに入り、チャネルにコマンドが現れるのを待ち、
//  それを実行し、そして次のコマンドを待ち始めるようにしてください。
pub fn server(receiver: Receiver<Command>) {}
