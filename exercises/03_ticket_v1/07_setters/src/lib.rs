// TODO: Add &mut-setters to the `Ticket` struct for each of its fields.
//   Make sure to enforce the same validation rules you have in `Ticket::new`!
//   Even better, extract that logic and reuse it in both places. You can use
//   private functions or private static methods for that.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {

    // validation priv funcs
   
    pub fn title_empt(title: &String) -> bool {
        title.is_empty()
    }

    pub fn title_len(title: &String) -> bool {
        title.len() > 50
    }

    pub fn desc_empt(description: &String) -> bool {
        description.is_empty()
    }

    pub fn desc_len(description: &String) -> bool {
        description.len() > 500
    }

    pub fn status_val(status: &String) -> bool {
        status != "To-Do" && status != "In Progress" && status != "Done"
    }
    

    pub fn new(title: String, description: String, status: String) -> Ticket {
        
        if Self::title_empt(&title) {
            panic!("Title cannot be empty");
        }
        if Self::title_len(&title) {
            panic!("Title cannot be longer than 50 bytes");
        }
        if Self::desc_empt(&description) {
            panic!("Description cannot be empty");
        }
        if Self::desc_len(&description) {
            panic!("Description cannot be longer than 50 bytes");
        }
        if Self::status_val(&status) {
            panic!("Invalid status value");
        }

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    // setters

    pub fn set_title(&mut self, new_title: String) {
        if Self::title_empt(&new_title) {
            panic!("Title cannot be empty");
        }
        if Self::title_len(&new_title) {
            panic!("Title cannot be longer than 50 bytes");
        }
        self.title = new_title;
    }

    pub fn set_description(&mut self, new_desc: String) {
        if Self::desc_empt(&new_desc) {
            panic!("Description cannot be empty");
        }
        if Self::desc_len(&new_desc) {
            panic!("Description cannot be longer than 500 bytes");
        }
        self.description = new_desc;
    }

    pub fn set_status(&mut self, new_status: String) {
        if Self::status_val(&new_status) {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }
        self.status = new_status;
    }

}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn works() {
        let mut ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        ticket.set_title("A new title".into());
        ticket.set_description("A new description".into());
        ticket.set_status("Done".into());

        assert_eq!(ticket.title(), "A new title");
        assert_eq!(ticket.description(), "A new description");
        assert_eq!(ticket.status(), "Done");
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_title("".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_description("".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_title(overly_long_title())
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_description(overly_long_description())
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_status("Funny".into());
    }
}
