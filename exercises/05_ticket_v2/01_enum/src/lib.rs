// TODO: `Ticket::status` の型として `Status` を使用してください。
//   必要に応じて、他のすべてのメソッドのシグネチャと実装を調整してください。

#[derive(Debug, PartialEq)]
// `derive` は再帰的です。すべてのフィールドが `PartialEq` を実装している場合にのみ、`PartialEq` を導出できます。
// `Debug` についても同じことが言えます。これを機能させるために `Status` に必要な処置を行ってください。
struct Ticket {
    title: String,
    description: String,
    status: String,
}

enum Status {
    // TODO: 不足しているバリアントを追加してください
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{valid_description, valid_title};

    #[test]
    fn test_partial_eq() {
        let title = valid_title();
        let description = valid_description();
        let ticket1 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::ToDo,
        };
        let ticket2 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::ToDo,
        };
        assert_eq!(ticket1, ticket2);
    }

    #[test]
    fn test_description_not_matching() {
        let title = valid_title();
        let status = Status::ToDo;
        let ticket1 = Ticket {
            title: title.clone(),
            description: "description".to_string(),
            status,
        };
        let ticket2 = Ticket {
            title: title.clone(),
            description: "description2".to_string(),
            status,
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_title_not_matching() {
        let description = valid_description();
        let status = Status::InProgress;
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: description.clone(),
            status,
        };
        let ticket2 = Ticket {
            title: "title2".to_string(),
            description: description.clone(),
            status,
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_status_not_matching() {
        let title = valid_title();
        let description = valid_description();
        let ticket1 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::InProgress,
        };
        let ticket2 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::Done,
        };
        assert_ne!(ticket1, ticket2);
    }
}
