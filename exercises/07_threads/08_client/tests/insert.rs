use client::data::{Status, TicketDraft};
use client::launch;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn insert_works() {
    // Notice how much simpler the test is now that we have a client to handle the details!
    let client = launch();
    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };
    let ticket_id = client.insert(draft.clone());

    let client2 = client.clone();
    let ticket = client2.get(ticket_id).unwrap();
    assert_eq!(ticket_id, ticket.id);
    assert_eq!(ticket.status, Status::ToDo);
    assert_eq!(ticket.title, draft.title);
    assert_eq!(ticket.description, draft.description);
}
