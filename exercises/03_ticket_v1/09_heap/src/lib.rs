pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: このセクションで学んだことに基づいて、`todo!()` を
//  それぞれの型の正しい**スタックサイズ**に置き換えてください。
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), todo!());
    }

    #[test]
    fn ticket_size() {
        // これは少し引っ掛け問題です！
        // 今回はたまたま「直感的な」答えが正解になりますが、
        // 一般的に構造体のメモリレイアウトはより複雑なトピックです。
        // 興味がある方は、The Rust Reference の「Type layout」セクション
        // https://doc.rust-lang.org/reference/type-layout.html で詳細を確認してください。
        assert_eq!(size_of::<Ticket>(), todo!());
    }
}
