use std::sync::mpsc::{Receiver, Sender};
use data::Ticket;
use store::TicketId;

use crate::store::TicketStore;

pub mod data;
pub mod store;

use crate::data::TicketDraft;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert { 
        draft: TicketDraft,
        response_sender: Sender<TicketId> 
    },
    Get { 
        id: TicketId,
        response_sender: Sender<Option<Ticket>>
    },
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() { // match receiver's command
            Ok(Command::Insert { // insert command needs a draft and a response_sender
                draft,
                response_sender,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_sender.send(id);
            }
            Ok(Command::Get {
                id,
                response_sender,
            }) => {
                let ticket = store.get(id);
                let _ = response_sender.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break
            },
        }
    }
}
