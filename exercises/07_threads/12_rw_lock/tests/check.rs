use rwlock::data::{Status, TicketDraft};
use rwlock::launch;
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
    let lock1 = ticket.read().unwrap();
    {
        let ticket = ticket.read().unwrap();
        assert_eq!(ticket_id, ticket.id);
        assert_eq!(ticket.status, Status::ToDo);
        assert_eq!(ticket.title, draft.title);
        assert_eq!(ticket.description, draft.description);
    }

    drop(lock1);

    let ticket = client.get(ticket_id).unwrap().unwrap();
    {
        let mut ticket = ticket.write().unwrap();
        ticket.status = Status::InProgress;
    }
}
