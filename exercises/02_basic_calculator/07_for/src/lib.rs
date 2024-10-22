// Rewrite the factorial function using a `for` loop.
pub fn factorial(n: u32) -> u32 {
    // todo!()
    if n == 0 || n == 1{
        return 1;
    }else{
        let mut res: u32 =  1;
        let mut i: u32 = 2;
        for i in 1..=n{
            res *= i;
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::factorial;

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
