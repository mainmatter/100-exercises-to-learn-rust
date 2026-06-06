use crate::Ticket;

fn create_todo_ticket(title: String, description: String) -> Ticket {
    Ticket::new(title, description, "To-Do".into())
}