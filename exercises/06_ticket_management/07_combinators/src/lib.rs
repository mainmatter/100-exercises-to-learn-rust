// TODO: Implement the `to_dos` method. It must return a `Vec` of references to the tickets
//  in `TicketStore` with status set to `Status::ToDo`.
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
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
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    pub fn to_dos(&self) -> Vec<&Ticket> {
        self.tickets
            .iter()
            .filter(|ticket| ticket.status == Status::ToDo)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn todos() {
        let mut store = TicketStore::new();

        let todo = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(todo.clone());

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let todos: Vec<&Ticket> = store.to_dos();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0], &todo);
    }
}
