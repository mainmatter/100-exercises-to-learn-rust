// TODO: based on what we just learned about ownership, it sounds like immutable references
//   are a good fit for our accessor methods.
//   Change the existing implementation of `Ticket`'s accessor methods to take a reference
//   to `self` as an argument, rather than taking ownership of it.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn status(&self) -> String {
        self.status.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;

    #[test]
    fn works() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        // If you change the signatures as requested, this should compile:
        // we can call these methods one after the other because they borrow `self`
        // rather than taking ownership of it.
        assert_eq!(ticket.title(), "A title");
        assert_eq!(ticket.description(), "A description");
        assert_eq!(ticket.status(), "To-Do");
    }
}
