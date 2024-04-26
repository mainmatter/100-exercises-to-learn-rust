// TODO: Implement `IndexMut<&TicketId>` and `IndexMut<TicketId>` for `TicketStore`.

use std::ops::{Index, IndexMut};
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
    counter: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TicketId(u64);

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
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
        self.tickets.push(ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.iter().find(|&t| t.id == id)
    }
}

impl Index<TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, index: TicketId) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl Index<&TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, index: &TicketId) -> &Self::Output {
        &self[*index]
    }
}

impl IndexMut<TicketId> for TicketStore {
    fn index_mut(&mut self, index: TicketId) -> &mut Self::Output {
        self.tickets.iter_mut().find(|t| t.id == index).unwrap()
    }
}

impl IndexMut<&TicketId> for TicketStore {
    fn index_mut(&mut self, index: &TicketId) -> &mut Self::Output {
        &mut self[*index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{Status, TicketDraft, TicketStore};
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let draft = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id = store.add_ticket(draft.clone());
        let ticket = &store[id];
        assert_eq!(draft.title, ticket.title);
        assert_eq!(draft.description, ticket.description);
        assert_eq!(ticket.status, Status::ToDo);

        let ticket = &mut store[id];
        ticket.status = Status::InProgress;

        let ticket = &store[id];
        assert_eq!(ticket.status, Status::InProgress);
    }
}
