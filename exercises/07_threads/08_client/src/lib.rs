use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: flesh out the client implementation.
pub struct TicketStoreClient { // The client using the ticket store has a sender transmitter with the command they want to use
    sender: Sender<Command>
}

impl TicketStoreClient {
    // Feel free to panic on all errors, for simplicity.
    pub fn insert(&self, draft: TicketDraft) -> TicketId { // to insert a ticket you need the draft and returns a ticket ID
        let (response_sender, response_receiver) = std::sync::mpsc::channel(); // create a channel
        self.sender // the ticket store's sender transmitter uses the Insert command and sents the ticket draft with the response sender
            .send(Command::Insert {
                draft, 
                response_channel: response_sender 
            })
            .unwrap(); // unwrap the result
        response_receiver.recv().unwrap() // receive the response and unwrap it 
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> { // to get a ticket by id you need the ticket ID
        let (response_sender, response_receiver) = std::sync::mpsc::channel();
        self.sender // the ticket store's sender transmitter uses the Get command
            .send(Command::Get {
                id, 
                response_channel: response_sender, // needs the transmitter of the response sender
            })
            .unwrap();
        response_receiver.recv().unwrap() // waits for a message to be received on the response_receiver
        // channel.
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

// No longer public! This becomes an internal detail of the library now.
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
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
