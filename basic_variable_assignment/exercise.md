---
Level (easy, medium, hard): easy
Tags (2-5): variables, mutability, references
GitHub user to credit: dpdresser
---

# Variable Assigment and Mutability

---

The basic syntax in Rust to assign data to a variable is:

```rust
let x = 5;
```

`x` acts as a container, storing the value `5`. `println!("{}", x + 1)` would display `6`.


In Rust variables are by default immutable, meaning they can't be changed once assigned. This is a safety feature of the language. For instance, the following code would not compile:

```rust
let x = 5;
x += 1;
println!("x is {}", x);
```
```Compiling playground v0.0.1 (/playground)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:3:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     x += 1;
  |     ^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++

For more information about this error, try `rustc --explain E0384`.
error: could not compile `playground` (bin "playground") due to 1 previous error
```

In order to allow Rust to mutate the value stored in a variable, you must use the `mut` keyword:

```rust
let mut x = 5;
x += 1;
println!("x is {}", x);
```

This would compile, and the output would be `x is 6`.
