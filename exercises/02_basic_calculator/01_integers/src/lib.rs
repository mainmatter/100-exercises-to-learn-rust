fn compute(a: u32, b: u32) -> u32 {
    let multiplier: u32 = 4;
    // ^ You could also omit `: u32` entirely.
    // The compiler will infer the correct type based on the usage below.
    a + b * multiplier
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
