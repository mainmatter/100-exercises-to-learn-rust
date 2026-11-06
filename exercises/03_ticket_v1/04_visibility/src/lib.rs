mod ticket {
    struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        fn new(title: String, description: String, status: String) -> Ticket {
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

// TODO: **例外的に**、この演習では `ticket` モジュールと `tests` モジュールの両方を変更します。
#[cfg(test)]
mod tests {
    // TODO: 以下の `use` 文に関するコンパイルエラーを解消するために、
    //  親モジュールに必要な `pub` 修飾子を追加してください。
    use super::ticket::Ticket;

    // ただし注意してください！`use` 文がコンパイルできるように可視性を変更した後、
    // この関数がコンパイルを通ってしまうのは望ましくありません！
    // 実際にコンパイルできないことを確認したら、コメントアウトしてください。
    fn should_not_be_possible() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());

        // この演習を実行しようとすると、次のエラーが表示されるはずです：
        //
        // error[E0616]: field `description` of struct `Ticket` is private
        //    |
        //    |              assert_eq!(ticket.description, "A description");
        //    |                         ^^^^^^^^^^^^^^^^^^
        //
        // TODO: 以下の行がコンパイルできないことを確認したら、
        //   その行をコメントアウトして次の演習に進んでください！
        assert_eq!(ticket.description, "A description");
    }

    fn encapsulation_cannot_be_violated() {
        // これも同様に不可能であるべきで、上記で遭遇したのと似たエラーが出るはずです。
        // （前のテストの問題のある行をコメントアウトした後にのみ、次のコンパイル段階で
        // コンパイルエラーが発生します！）
        //
        // これにより、`Ticket::new` が `Ticket` インスタンスを取得する唯一の方法になったことが証明されます。
        // 不正なタイトルや説明文を持つチケットを作成することは不可能です！
        //
        // TODO: 以下がコンパイルできないことを確認したら、
        //   これらの行をコメントアウトして次の演習に進んでください！
        let ticket = Ticket {
            title: "A title".into(),
            description: "A description".into(),
            status: "To-Do".into(),
        };
    }
}
