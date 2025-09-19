mod ticket {
    pub struct Ticket {
        pub title: String,
        pub description: String,
        pub status: String,
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
    }
}

// TODO: **Exceptionally**, you'll be modifying both the `ticket` module and the `tests` module
//  in this exercise.
#[cfg(test)]
mod tests {
    // TODO: Add the necessary `pub` modifiers in the parent module to remove the compiler
    //  errors about the use statement below.
    use super::ticket::Ticket;

    // Be careful though! We don't want this function to compile after you have changed
    // visibility to make the use statement compile!
    // Once you have verified that it indeed doesn't compile, comment it out.
    fn should_not_be_possible() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());

        // You should be seeing this error when trying to run this exercise:
        //
        // error[E0616]: field `description` of struct `Ticket` is private
        //    |
        //    |              assert_eq!(ticket.description, "A description");
        //    |                         ^^^^^^^^^^^^^^^^^^
        //
        // TODO: Once you have verified that the below does not compile,
        //   comment the line out to move on to the next exercise!
        assert_eq!(ticket.description, "A description");
    }

    fn encapsulation_cannot_be_violated() {
        // This should be impossible as well, with a similar error as the one encountered above.
        // (It will throw a compilation error only after you have commented the faulty line
        // in the previous test - next compilation stage!)
        //
        // This proves that `Ticket::new` is now the only way to get a `Ticket` instance.
        // It's impossible to create a ticket with an illegal title or description!
        //
        // TODO: Once you have verified that the below does not compile,
        //   comment the lines out to move on to the next exercise!
        let ticket = Ticket {
            title: "A title".into(),
            description: "A description".into(),
            status: "To-Do".into(),
        };
    }
}
