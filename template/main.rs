fn is_even(n: u32) -> bool {
    // rename this function (and signature) and add your code exercise here
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        // update this test (name and content) to match the function
        assert_eq!(is_even(2), true);
        assert_eq!(is_even(3), false);
        assert_eq!(is_even(0), true);
    }

    // and repeat last block for more tests
}
