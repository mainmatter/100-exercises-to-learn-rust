use patch::data::{Status, TicketDraft, TicketPatch};
use patch::launch;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn works() {
    let client = launch(5);
    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };
    let ticket_id = client.insert(draft.clone()).unwrap();

    let ticket = client.get(ticket_id).unwrap().unwrap();
    assert_eq!(ticket_id, ticket.id);
    assert_eq!(ticket.status, Status::ToDo);
    assert_eq!(ticket.title, draft.title);
    assert_eq!(ticket.description, draft.description);

    let patch = TicketPatch {
        id: ticket_id,
        title: None,
        description: None,
        status: Some(Status::InProgress),
    };
    client.update(patch).unwrap();

    let ticket = client.get(ticket_id).unwrap().unwrap();
    assert_eq!(ticket.id, ticket_id);
    assert_eq!(ticket.status, Status::InProgress);
}
