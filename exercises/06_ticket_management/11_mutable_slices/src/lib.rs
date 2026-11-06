// TODO: スライス内のすべての `i32` を2乗する `squared` という名前の関数を定義してください。
//  スライスはその場で（in placeで）直接変更される必要があります。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut s = vec![];
        squared(&mut s);
        assert_eq!(s, vec![]);
    }

    #[test]
    fn one() {
        let mut s = [2];
        squared(&mut s);
        assert_eq!(s, [4]);
    }

    #[test]
    fn multiple() {
        let mut s = vec![2, 4];
        squared(&mut s);
        assert_eq!(s, vec![4, 16]);
    }
}
