// TODO: Implement `Debug`, `Display` and `Error` for the `TicketNewError` enum.
//  When implementing `Display`, you may want to use the `write!` macro from Rust's standard library.
//  The docs for the `std::fmt` module are a good place to start and look for examples:
//  https://doc.rust-lang.org/std/fmt/index.html#write

#[derive(Debug)]
enum TicketNewError {
    TitleError(String),
    DescriptionError(String),
}

impl std::fmt::Display for TicketNewError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TicketNewError::TitleError(msg) => write!(f, "Title error: {}", msg),
            TicketNewError::DescriptionError(msg) => write!(f, "Description error: {}", msg),
        }
    }
}

impl std::error::Error for TicketNewError {}

// TODO: `easy_ticket` should panic when the title is invalid, using the error message
//   stored inside the relevant variant of the `TicketNewError` enum.
//   When the description is invalid, instead, it should use a default description:
//   "No description provided".
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description.clone(), status.clone()) {
        Ok(ticket) => ticket,
        Err(err) => match err {
            TicketNewError::TitleError(_) => panic!("{err}"),
            TicketNewError::DescriptionError(_) => {
                Ticket::new(title, "Description not provided".to_string(), status).unwrap()
            }
        },
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleError(
                "Title cannot be empty".to_string(),
            ));
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleError(
                "Title cannot be longer than 50 characters".to_string(),
            ));
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionError(
                "Description cannot be empty".to_string(),
            ));
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionError(
                "Description cannot be longer than 500 characters".to_string(),
            ));
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};
    use static_assertions::assert_impl_one;

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 characters")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    fn display_is_correctly_implemented() {
        let ticket = Ticket::new("".into(), valid_description(), Status::ToDo);
        assert_eq!(
            format!("{}", ticket.unwrap_err()),
            "Title error: Title cannot be empty"
        );
    }

    assert_impl_one!(TicketNewError: std::error::Error);
}
