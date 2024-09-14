fn count_vowels(s: &str) -> usize {
    s.chars().filter(|&c| "aeiouAEIOU".contains(c)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_vowels() {
        assert_eq!(count_vowels("hello"), 2);
        assert_eq!(count_vowels("rust"), 1);
        assert_eq!(count_vowels("RUST"), 1);
        assert_eq!(count_vowels("aeiouAEIOU"), 10);
        assert_eq!(count_vowels("bcdfgh"), 0);
    }
}
