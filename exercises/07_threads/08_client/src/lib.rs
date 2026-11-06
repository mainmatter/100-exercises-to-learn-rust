use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: クライアントの実装を完成させてください。
pub struct TicketStoreClient {}

impl TicketStoreClient {
    // 簡潔にするため、すべてのエラーに対して遠慮なくパニックさせてください。
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        todo!()
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        todo!()
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    todo!()
}

// もはや公開（pub）ではありません！これでライブラリの内部実装の詳細になりました。
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // 送信側（Sender）がもう存在しないため、安全に break して
                // サーバーをシャットダウンできます。
                break;
            }
        }
    }
}
