use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Provide an `iter` method that returns an iterator over `&Ticket` items.
//
// Hint: just like in the previous exercise, you want to delegate the iteration to
//   the `Vec<Ticket>` field in `TicketStore`. Look at the standard library documentation
//   for `Vec` to find the right type to return from `iter`.
#[derive(Clone, Default)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

impl IntoIterator for TicketStore {
    type Item = Ticket;

    type IntoIter = std::vec::IntoIter<Ticket>;

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.into_iter()
    }
}

impl<'a> IntoIterator for &'a TicketStore {
    type Item = &'a Ticket;

    type IntoIter = std::slice::Iter<'a, Ticket>;

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    title: TicketTitle,
    description: TicketDescription,
    status: Status,
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

    pub fn iter(&self) -> impl Iterator<Item = &Ticket> {
        self.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<&Ticket> = store.iter().collect();
        let tickets2: Vec<&Ticket> = store.iter().collect();
        assert_eq!(tickets, tickets2);
    }
}
