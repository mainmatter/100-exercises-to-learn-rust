// Define a trait named `IsEven` that has a method `is_even` that returns a `true` if `self` is
// even, otherwise `false`.
//
// Then implement the trait for `u32` and `i32`.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_is_even() {
        assert!(42.is_even());
        assert!(!43.is_even());
    }

    #[test]
    fn test_i32_is_even() {
        assert!(42.is_even());
        assert!(!43.is_even());
        assert!(0.is_even());
        assert!(!(-1).is_even());
    }
}
