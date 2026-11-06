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
    fn u16_ref_size() {
        assert_eq!(size_of::<&u16>(), todo!());
    }

    #[test]
    fn u64_mut_ref_size() {
        assert_eq!(size_of::<&mut u64>(), todo!());
    }

    #[test]
    fn ticket_ref_size() {
        assert_eq!(size_of::<&Ticket>(), todo!());
    }
}
