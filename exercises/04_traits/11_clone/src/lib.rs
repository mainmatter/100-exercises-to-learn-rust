// TODO: コードがコンパイルできるように、必要な `Clone` の実装（および呼び出し）を追加してください。

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    (ticket, ticket.summary())
}

pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}
