// TODO: Implement the `in_progress` method. It must return an iterator over the tickets in
//  `TicketStore` with status set to `Status::InProgress`.
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

    /*
        The previous example introduces a new heap allocation every time to_dos is called, which may
        be unecessary depending on what the called needs to do wit hthe result. This will instead 
        return an iterator instead of a Vec, thus empowering the caller to decide whether to collect
        the results into a Vec or just iterate over them.

        impl Trait: is a feature that allows you to return a type without specifying its name. You just
        declare what trait(s) the type implements, and Rust figures out the rest.

        In this case we want to return an iterator of references to Tickets:
     */

    pub fn in_progress(&self) -> impl Iterator<Item = &Ticket> {
        self.tickets.iter().filter((|ticket| ticket.status == Status::InProgress))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn in_progress() {
        let mut store = TicketStore::new();

        let todo = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(todo);

        let in_progress = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(in_progress.clone());

        let in_progress_tickets: Vec<&Ticket> = store.in_progress().collect();
        assert_eq!(in_progress_tickets.len(), 1);
        assert_eq!(in_progress_tickets[0], &in_progress);
    }
}
