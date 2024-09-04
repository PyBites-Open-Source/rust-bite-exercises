---
Level (easy, medium, hard): medium
Tags (2-5): structs, methods, strings
GitHub user to credit: bbelderbos
---

# Using Structs in Rust

Update the `Person` struct to model a person with `name`, `age`, and `email` fields. Implement a method `greet` that returns a greeting string using the struct's fields.

## Example

Given a `Person` instance with the following fields, it should return the following greeting:

```rust
let person = Person {
    name: String::from("John"),
    age: 30,
    email: String::from("john@example.com"),
};

let greeting = person.greet();  // Hello, my name is John and I am 30 years old. You can contact me at john@example.com
```
