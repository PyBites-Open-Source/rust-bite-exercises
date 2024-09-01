fn my_function(...) -> ... {
    // rename function and add code here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        assert_eq!(my_function(...), ...);
    }

    // repeat last block for more tests
}
