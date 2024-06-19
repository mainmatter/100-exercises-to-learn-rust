// The input parameters should have the same type of the return type.
fn compute(a: u32, b: u32) -> u32 {
    // Don't touch the function body.
    a + b * 2
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 5);
    }
}