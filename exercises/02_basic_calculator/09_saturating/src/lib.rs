pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        // オーバーフローしてラップアラウンドするのではなく、
        // u32 の最大値で止まるように飽和乗算（saturating multiplication）を使用してください
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        assert_eq!(factorial(20), u32::MAX);
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
