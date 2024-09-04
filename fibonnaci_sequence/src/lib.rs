fn fibonacci(n: i32) -> u32 {
    match n {
        n if n < 0 => panic!("Negative input is not allowed"), // Handle negative input by panicking
        0 => 0,                                                // Base case: fibonacci(0) = 0
        1 => 1,                                                // Base case: fibonacci(1) = 1
        _ => fibonacci(n - 1) + fibonacci(n - 2),              // Recursive case
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
    }

    #[test]
    #[should_panic]
    fn test_fibonacci_negative() {
        fibonacci(-1); // Should panic for negative input
    }
}
