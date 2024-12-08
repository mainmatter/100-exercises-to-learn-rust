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

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    // SETTERS

    // FIRST APPROACH: using mut self as input

    /*
        The function takes ownership of self, changes the title, and returns the modified
        Ticket instance.

        self-setters work nicely when you need to change multiple fields at once: you can
        chain multiple calls together!
     */

    //  pub fn set_title(mut self, new_title: String) -> Self {
    //     if new_title.is_empty() {
    //         panic!("Title cannot be empty");
    //     }
    //     if new_title.len() > 50 {
    //         panic!("Title cannot be longer than 50 bytes");
    //     }
    //     self.title = new_title;
    //     self
    // }

    // pub fn set_description(mut self, new_description: String) -> Self {
    //     if new_description.is_empty() {
    //         panic!("Description cannot be empty");
    //     }
    //     if new_description.len() > 500 {
    //         panic!("Description cannot be longer than 500 bytes");
    //     }
    //     self.description = new_description;
    //     self
    // }

    // pub fn set_status(mut self, new_status: String) -> Self {
    //     if new_status != "To-Do" && new_status != "In Progress" && new_status != "Done" {
    //         panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
    //     }
    //     self.status = new_status;
    //     self
    // }

    /*
        Since set_title takes ownership of self (i.e. it consumes it), we need to 
        reassign the result to a variable. In the example above we take advantage of 
        'variable shadowing' to reuse the same variable name: when you declare a new 
        variable with the same name as an existing one, the new variable 'shadows'
        the old one. 
     */

    // USAGE

    /*
        let ticket = ticket
            .set_title("New title".into())
            .set_description("New description".into())
            .set_status("In Progress".into());
     */
    
    // SECOND APPROACH: using &mut self, looks like this:

    /*
        Ownership stays with the caller, so the original ticket variable is still valid.
        We don't need to reassign the result. We need to mark ticket as mutable though,
        because we're taking a mutable reference to it. 
    
        Downside: you can't chain multipe calls together. Since they don't return the 
        modified Ticket instance, you can't call another setter on the result of the first
        one. you have to call each setter separately. 
     */


    pub fn set_title(&mut self, new_title: String) {
        if new_title.is_empty() {
            panic!("Title cannot be empty");
        }
        if new_title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        self.title = new_title;
    }

    pub fn set_description(&mut self, new_description: String) {
        if new_description.is_empty() {
            panic!("Description cannot be empty");
        }
        if new_description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
        self.description = new_description;
    }

    pub fn set_status(&mut self, new_status: String) {
        if new_status != "To-Do" && new_status != "In Progress" && new_status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }
        self.status = new_status;
    }

    // USAGE

    /*
        let mut ticket = Ticket::new(
            "Title".into(),
            "Description".into(),
            "To-Do".into()
        );

        ticket.set_title("New title".into());
        ticket.set_description("New description".into());
        ticket.set_status("In Progress".into());

     */
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
