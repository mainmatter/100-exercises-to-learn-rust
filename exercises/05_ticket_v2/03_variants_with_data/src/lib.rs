// TODO: Implement `Ticket::assigned_to`.
//  Return the name of the person assigned to the ticket, if the ticket is in progress.
//  Panic otherwise.

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Ticket {
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

        Ticket {
            title,
            description,
            status,
        }
    }
    pub fn assigned_to(&self) -> &str {
        match &self.status {
            Status::InProgress { assigned_to } => assigned_to,
            _ => panic!("Only `In-Progress` tickets can be assigned to someone"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Only `In-Progress` tickets can be assigned to someone")]
    fn test_todo() {
        let ticket = Ticket::new(valid_title(), valid_description(), Status::ToDo);
        ticket.assigned_to();
    }

    #[test]
    #[should_panic(expected = "Only `In-Progress` tickets can be assigned to someone")]
    fn test_done() {
        let ticket = Ticket::new(valid_title(), valid_description(), Status::Done);
        ticket.assigned_to();
    }

    #[test]
    fn test_in_progress() {
        let ticket = Ticket::new(
            valid_title(),
            valid_description(),
            Status::InProgress {
                assigned_to: "Alice".to_string(),
            },
        );
        assert_eq!(ticket.assigned_to(), "Alice");
    }
}
