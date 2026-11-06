use ticket_fields::{TicketDescription, TicketTitle};

// TODO: チケットストアの設計を始めましょう！
//  最初のタスク：`for` ループを使って `TicketStore` に含まれるすべてのチケットを反復処理できるように、
//  `TicketStore` に `IntoIterator` を実装してください。
//
// ヒント：このケースでは `Iterator` トレイトを自分で実装する必要はありません。
//   反復処理を `TicketStore` 内の `Vec<Ticket>` フィールドに*委譲 (delegate)* しましょう。
//   `into_iter` から返すべき正しい型を見つけるには、`Vec` の標準ライブラリドキュメントを確認してください。
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<_> = store.clone().into_iter().collect();
        assert_eq!(tickets, store.tickets);
    }
}
