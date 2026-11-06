// TODO: `TicketStore::add_ticket` のシグネチャを、`impl Trait` 構文ではなく
//  ジェネリック型パラメータを使用するように書き直してください。

use ticket_fields::{TicketDescription, TicketTitle};

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

    // `ticket` の型パラメータとして `Into<Ticket>` を使用すると、失敗することなく `Ticket` に
    // 変換できる任意の型をメソッドが受け入れられるようになります。
    // これにより、呼び出し箇所から `.into()` という構文のノイズが取り除かれるため、
    // メソッドが使いやすくなります。ただし、コンパイラのエラーメッセージの質が悪化する可能性はあります。
    pub fn add_ticket(&mut self, ticket: impl Into<Ticket>) {
        self.tickets.push(ticket.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    struct TicketDraft {
        pub title: TicketTitle,
        pub description: TicketDescription,
    }

    impl From<TicketDraft> for Ticket {
        fn from(draft: TicketDraft) -> Self {
            Self {
                title: draft.title,
                description: draft.description,
                status: Status::ToDo,
            }
        }
    }

    #[test]
    fn generic_add() {
        let mut store = TicketStore::new();
        // `add_ticket` が引数の位置で `impl Trait` 構文を使用している場合、これはコンパイルできません。
        store.add_ticket::<TicketDraft>(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        });
    }
}
