use crate::{TicketDescription, TicketTitle};
use common::{valid_description, valid_title};

/// A function to generate a valid ticket title,
/// for test purposes.
pub fn ticket_title() -> TicketTitle {
    valid_title().try_into().unwrap()
}

/// A function to generate a valid ticket description,
/// for test purposes.
pub fn ticket_description() -> TicketDescription {
    valid_description().try_into().unwrap()
}
