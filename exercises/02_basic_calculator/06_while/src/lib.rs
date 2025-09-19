// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    // The factorial of a number greater than 12 would overflow a u32.
    // We use an assertion to prevent this from happening, which will cause
    // the program to panic if the condition is not met.
    assert!(n <= 12); // 13! overflows

    // We need a mutable variable to hold the current number, so we shadow the
    // original `n` parameter.
    let mut n = n;
    // We start with an accumulator of 1.
    // The factorial of 0 is 1, and this is also the multiplicative identity.
    let mut accum = 1;

    // We loop as long as n is greater than 1.
    while n > 1 {
        // We multiply the accumulator by the current number.
        accum *= n;
        // We decrement the number.
        n -= 1;
    }

    // We return the final accumulated value.
    accum
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
