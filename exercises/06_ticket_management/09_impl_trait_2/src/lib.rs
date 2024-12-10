// TODO: Rework the signature of `TicketStore::add_ticket` to use a generic type parameter rather
//  than `impl Trait` syntax.

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

    /*
        In the previous section, we saw how impl Trait can be used to return a type without 
        specifying its name. The same syntax can also be used in argument position:

        fn print_iter(iter: impl Iterator<Item = i32>) {
            for i in iter {
                println!("{}", i);
            }
        }


        print_iter takes an iterator of i32s and prints each element. 
        When used in argument position, impl Trait is equivalent to a generic parameter with
        a trait bound: 
    
        fn print_iter<T>(iter: T) 
        where
            T: Iterator<Item = i32>
        {
            for i in iter {
                println!("{}", i);
            }
        }

        Downsides:
        As a rule of thumb, prefer generics over impl Trait in argument position. Generics allow 
        the called to explicitly specify the type of the argument, using the turbofish syntax
        (::<>), which can be useful for disambiguation. That's not the case with impl Trait. 
    
     */


    // Using `Into<Ticket>` as the type parameter for `ticket` allows the method to accept any type
    // that can be infallibly converted into a `Ticket`.
    // This can make it nicer to use the method, as it removes the syntax noise of `.into()`
    // from the calling site. It can worsen the quality of the compiler error messages, though.

    // pub fn add_ticket(&mut self, ticket: impl Into<Ticket>) {
    //     self.tickets.push(ticket.into());
    // }

    pub fn add_ticket<T: Into<Ticket>>(&mut self, ticket: T) {
        self.tickets.push(ticket.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    struct TicketDraft {
        pub title: TicketTitle,
        pub description: TicketDescription,
    }

    impl From<TicketDraft> for Ticket {
        fn from(draft: TicketDraft) -> Self {
            Self {
                title: draft.title,
                description: draft.description,
                status: Status::ToDo,
            }
        }
    }

    #[test]
    fn generic_add() {
        let mut store = TicketStore::new();
        // This won't compile if `add_ticket` uses `impl Trait` syntax in argument position.
        store.add_ticket::<TicketDraft>(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        });
    }
}
