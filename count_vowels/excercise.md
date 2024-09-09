---
Level (easy, medium, hard): easy
Tags (2-5): strings
GitHub user to credit: ToHellAndBeck
---

# Vowel Counter

---

This Rust function, `count_vowels`, takes a string slice (`&str`) as input and returns the number of vowels (both lowercase and uppercase) present in the string. It works by iterating over each character in the input string and checking if the character is a vowel (one of `"a, e, i, o, u, A, E, I, O, U"`). The function uses Rust’s built-in iterator methods—`chars()` to convert the string into a sequence of characters and `filter()` to keep only the vowels. The `count()` method is then used to return the number of filtered vowels.

This function is useful in scenarios where you need to analyze text for vowel counts, such as in educational tools, text processing, or language-based applications.

Example usage:
```rust
let input = "Hello, Rustacean!";
let vowel_count = count_vowels(input);
println!("The number of vowels in '{}' is: {}", input, vowel_count);
```



