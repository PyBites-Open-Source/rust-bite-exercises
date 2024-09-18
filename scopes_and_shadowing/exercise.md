---
Level (easy, medium, hard): easy
Tags (2-5): scopes, shadowing, mutability, references, lifetimes
GitHub user to credit: dpdresser
---

# Scopes and Shadowing

Scope is a region of code that defines the lifetime of variables, usually defined by curly braces `{}`. This includes the body of a function, which is also a scope.

In Rust, variables can be shadowed, meaning you can declare a new variable with the same name as a previous variable. This new variable will "shadow" the previous one within its scope. Shadowing allows you to reuse variable names and is different from mutability.

Here's an example of scopes and shadowing:

```rust
let x = 5;
let y = 7;
{
    let x = x + 1; // Reads the outer x value and shadows the outer x in a new variable.
    {
        let x = String::from("this is x!"); // x is shadowed again, but with a new type.
        println!("Inner-inner x: {}", x); // Prints "Inner-inner x: this is x!"
        println!("Outer y: {}", y); // Prints "Outer y: 7" -- variables declared in a scope are visible in any nested scope if not shadowed.
    }
    println!("Inner x: {}", x); // Prints "Inner x: 6"
}
println!("Outer x: {}", x); // Prints "Outer x: 5"
```

Scopes can be nested in as many levels as you desire, just be careful or things can get confusing. Shadowing can also change the type of a variable. 

Scope is an important concept in Rust, because the lifetime of a variable is determined by its scope. When a given scope ends, variables declared inside that scope are no longer accessible, and the memory used by those variables can be reclaimed. Accessing a variable in an outer scope is not allowed (without a reference) when it is shadowed by another variable in the current scope.
