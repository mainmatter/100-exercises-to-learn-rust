// TODO: `Status` に対して `TryFrom<String>` と `TryFrom<&str>` を実装してください。
//  パースは大文字・小文字を区別しない（case-insensitive）ようにする必要があります。

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);

        let empty = String::new();
        assert!(Status::try_from(empty).is_err());

        let invalid = "InValid".to_string();
        assert!(Status::try_from(invalid).is_err());
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);

        assert!(Status::try_from("").is_err());
        assert!(Status::try_from("invalid").is_err());
    }
}
