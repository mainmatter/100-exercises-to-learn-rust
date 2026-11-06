// TODO: アクセサメソッドを介して `title` と `description` が返されるときは常に、
//   それらが正規化されている（つまり、先頭と末尾の空白が削除されている）必要があります。
//   Rustの標準ライブラリにはこれに役立つメソッドがありますが、`String` のドキュメントには見当たりません。
//   それがどこで定義されており、どのように使用すればよいかわかりますか？

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn title(&self) -> &str {
        todo!()
    }

    pub fn description(&self) -> &str {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let ticket = Ticket {
            title: "   A title ".to_string(),
            description: " A description   ".to_string(),
            status: "To-Do".to_string(),
        };

        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
