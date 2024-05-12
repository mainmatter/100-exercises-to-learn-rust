use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::data::{Status, Ticket, TicketDraft};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Arc<RwLock<Ticket>>>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        let ticket = Arc::new(RwLock::new(ticket));
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<Arc<RwLock<Ticket>>> {
        self.tickets.get(&id).cloned()
    }
}
