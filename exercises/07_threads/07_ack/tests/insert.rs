use response::data::{Status, Ticket, TicketDraft};
use response::store::TicketId;
use response::{launch, Command};
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn insert_works() {
    let sender = launch();
    let (response_sender, response_receiver) = std::sync::mpsc::channel();

    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };
    let command = Command::Insert {
        draft: draft.clone(),
        response_sender,
    };

    sender
        .send(command)
        // If the thread is no longer running, this will panic
        // because the channel will be closed.
        .expect("Did you actually spawn a thread? The channel is closed!");

    let ticket_id: TicketId = response_receiver.recv().expect("No response received!");

    let (response_sender, response_receiver) = std::sync::mpsc::channel();
    let command = Command::Get {
        id: ticket_id,
        response_sender,
    };
    sender
        .send(command)
        .expect("Did you actually spawn a thread? The channel is closed!");

    let ticket: Ticket = response_receiver
        .recv()
        .expect("No response received!")
        .unwrap();
    assert_eq!(ticket_id, ticket.id);
    assert_eq!(ticket.status, Status::ToDo);
    assert_eq!(ticket.title, draft.title);
    assert_eq!(ticket.description, draft.description);
}
