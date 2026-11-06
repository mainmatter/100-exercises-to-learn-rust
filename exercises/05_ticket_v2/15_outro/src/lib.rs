// TODO: このクレート内の各モジュールにやるべき作業があります！
mod description;
mod status;
mod title;

// Rustの一般的なパターンとして、コードを複数の（非公開）モジュールに分割し、
// それらのモジュールの公開部分をクレートのルートで再エクスポート（re-export）するというものがあります。
//
// これにより、コードを自由に整理できるようにしつつ、クレートの内部構造を
// ユーザーから隠蔽することができます。
pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone)]
// フィールドを非公開にする必要はもうありません！
// 各フィールドが独自のバリデーションロジックをカプセル化しているため、`Ticket` のユーザーが
// 構造体の不変条件を壊すような方法でフィールドを変更してしまうリスクはありません。
//
// ただし注意してください。もし複数のフィールドにまたがる不変条件がある場合は、
// それらの不変条件が維持されることを保証する必要があり、再びフィールドを非公開にする必要があります。
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
