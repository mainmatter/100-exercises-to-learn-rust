struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    // TODO: `new` 関数を実装してください。
    //  以下の要件を満たす必要があります：
    //   - `To-Do`、`In Progress`、`Done` のステータスのみが許可されます。
    //   - `title` と `description` フィールドは空であってはなりません。
    //   - `title` の長さは最大50バイトまでです。
    //   - `description` の長さは最大500バイトまでです。
    //  いずれかの要件が満たされない場合、メソッドはパニックする必要があります。
    //  必要なパニックメッセージはテスト内で確認できます。
    //
    // これまでの演習で学んだことに加えて、いくつかの `String` のメソッドを使用する必要があります。
    // 最も適切な選択肢を見つけるために、Rust標準ライブラリのドキュメントを活用してください -> https://doc.rust-lang.org/std/string/struct.String.html
    fn new(title: String, description: String, status: String) -> Self {
        todo!();
        Self {
            title,
            description,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
