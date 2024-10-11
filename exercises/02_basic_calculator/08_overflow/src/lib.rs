// Customize the `dev` profile to wrap around on overflow.
// Check Cargo's documentation to find out the right syntax:
// https://doc.rust-lang.org/cargo/reference/profiles.html
//
// For reasons that we'll explain later, the customization needs to be done in the `Cargo.toml`
// at the root of the repository, not in the `Cargo.toml` of the exercise.

pub fn factorial(n: u32) -> Option<u32> {
    let mut result: u32 = 1;
    for i in 1..=n {
        result = result.checked_mul(i)?;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        // 20! is 2432902008176640000, which is too large to fit in a u32
        // With the default dev profile, this will panic when you run `cargo test`
        // We want it to wrap around instead
        //assert_eq!(factorial(20), 2_192_834_560);
        assert_eq!(factorial(20), None);
        //                           ☝️
        // A large number literal using underscores to improve readability!
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), Some(1));
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), Some(1));
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), Some(2));
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), Some(120));
    }
}
