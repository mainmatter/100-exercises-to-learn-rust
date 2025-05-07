// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    // The `todo!()` macro is a placeholder that the compiler
    // interprets as "I'll get back to this later", thus
    // suppressing type errors.
    // It panics at runtime.
    todo!();
    if n == 0{return 0;}
    let ans = 1 as u32;
    while n >= 1{
        ans *= n;
        n -= 1;
    }
    ans
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
