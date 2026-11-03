// Define a function named `factorial` that, given a non-negative integer `n`,
// returns `n!`, the factorial of `n`.
//
// The factorial of `n` is defined as the product of all positive integers up to `n`.
// For example, `5!` (read "five factorial") is `5 * 4 * 3 * 2 * 1`, which is `120`.
// `0!` is defined to be `1`.
//
// We expect `factorial(0)` to return `1`, `factorial(1)` to return `1`,
// `factorial(2)` to return `2`, and so on.
//
// Use only what you learned! No loops yet, so you'll have to use recursion!

// factorial 
// 5! = 5 x (5-1) x (5-2) x (5-3) x (5-4) x (5-5)
// n! = n x (n-1) x (n-2) x (n-3) x (n-4) x (n-n)
// 0! = 1
// 1! = 1 X (1-1) 
// 2! = 2 x (2-1) X (2-2)
// 2! = 3 x (3-1) x (3-2) x (3-3)
// 4! = 4 x (4-1) x (4-2) x (4-3) x (4-4)
// 5! = 5 x (5-1) x (5-2) x (5-3) X (5-4) x (5x5)
// n! = n x (n-1)!


fn factorial(n: u32) -> u32{

    if n > 0 {
        n * factorial(n-1)
    }
    else{
        return 1
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
