// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
pub fn fibonacci(n: u32) -> u32 {
    let mut memo = vec![Option::<u32>::None; n as usize + 1];
    fibonacci_helper(n, &mut memo)
}

fn fibonacci_helper(n: u32, memo: &mut Vec<Option<u32>>) -> u32 {
    match n {
        0 => return 0,
        1 => return 1,
        _ => (),
    }

    if let Some(val) = memo[n as usize] {
        val
    } else {
        let result = fibonacci_helper(n - 1, memo) + fibonacci_helper(n - 2, memo);
        memo[n as usize] = Some(result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
