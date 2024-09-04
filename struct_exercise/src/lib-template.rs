struct Person {
    // Define fields for name, age, and email
}

impl Person {
    // Implement a method to return a greeting string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_greet() {
        let person = Person {
            name: String::from("Alice"),
            age: 28,
            email: String::from("alice@example.com"),
        };

        // Test the greet method
        let greeting = person.greet();
        assert_eq!(
            greeting,
            "Hello, my name is Alice and I am 28 years old. You can contact me at alice@example.com"
        );
    }

    #[test]
    fn test_person_greet_different_person() {
        let person = Person {
            name: String::from("Bob"),
            age: 45,
            email: String::from("bob@example.com"),
        };

        // Test the greet method for a different person
        let greeting = person.greet();
        assert_eq!(
            greeting,
            "Hello, my name is Bob and I am 45 years old. You can contact me at bob@example.com"
        );
    }
}
