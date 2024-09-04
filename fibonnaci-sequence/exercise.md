---
Level (easy, medium, hard): easy
Tags (2-5): recursion, math, functions
GitHub user to credit: pybites
---

# Fibonacci Sequence

Implement the `fibonacci` function that takes an integer `n` as input and returns the `n`th Fibonacci number.

The Fibonacci sequence is a series of numbers where each number (after the first two) is the sum of the two preceding ones. The sequence starts with 0 and 1. For example, the sequence begins: 0, 1, 1, 2, 3, 5, 8, ...

If the input is a negative number, the function should handle it by panicking with a meaningful error message.

## Examples

1. Given a valid input `n = 5`, the function should return `5`:

    ```rust
    let result = fibonacci(5);  // 5
    ```

2. Given another valid input `n = 10`, the function should return `55`:

    ```rust
    let result = fibonacci(10);  // 55
    ```

3. If the input is a negative number, such as `n = -1`, the function should panic with an error message:

    ```rust
    fibonacci(-1);  // Should panic with "Negative input is not allowed"
    ```
