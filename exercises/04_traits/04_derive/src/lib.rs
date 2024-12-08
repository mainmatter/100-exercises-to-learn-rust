// TODO: A (derivable) trait implementation is missing for this exercise to compile successfully.
//   Fix it!
//
// # `Debug` primer
//
// `Debug` returns a representation of a Rust type that's suitable for debugging (hence the name).
// `assert_eq!` requires `Ticket` to implement `Debug` because, when the assertion fails, it tries to
// print both sides of the comparison to the terminal.
// If the compared type doesn't implement `Debug`, it doesn't know how to represent them!

/*
    Rust macros are CODE GENERATORS.
    They generate new Rust code based on the input you provide, and that generated code
    is then compiled alongside the rest of your program. some macros are built into Rust's
    standard library, but you can also write your own. 

    A derive macro is a an attribute ontop of a struct. 
    Derive macros are used to automate the implementation of common (and 'obvious') traits
    for custom types. 

    Putting #[derive(PartialEq)] ontop of the Ticket struct, will automatically implement
    the PartialEq trait for Ticket. 

*/

#[derive(PartialEq, Debug)]
struct Ticket {
    title: String,
    description: String,
    status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
        let title = "title";
        let description = "description";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        assert_eq!(ticket1, ticket2);
    }

    #[test]
    fn test_description_not_matching() {
        let title = "title";
        let status = "To-Do";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: "description".to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: "description2".to_string(),
            status: status.to_string(),
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_title_not_matching() {
        let status = "To-Do";
        let description = "description";
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        let ticket2 = Ticket {
            title: "title2".to_string(),
            description: description.to_string(),
            status: status.to_string(),
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_status_not_matching() {
        let title = "title";
        let description = "description";
        let ticket1 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: "status".to_string(),
        };
        let ticket2 = Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status: "status2".to_string(),
        };
        assert_ne!(ticket1, ticket2);
    }
}
