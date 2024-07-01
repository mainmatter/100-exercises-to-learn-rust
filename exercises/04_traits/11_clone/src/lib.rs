// TODO: add the necessary `Clone` implementations (and invocations)
//  to get the code to compile.

use std::iter::Sum;

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    (ticket.clone(), ticket.summary())
}

pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Clone for Ticket {
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            description: self.description.clone(),
            status: self.status.clone(),
        }
    }
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}

impl Clone for Summary {
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            status: self.status.clone(),
        }
    }
}
