use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

fn serialize_person(person: &Person) -> String {
    // return a JSON string from the given Person instance
}

fn deserialize_person(json: &str) -> Person {
    // return a Person instance from the given JSON string
}
