use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

fn serialize_person(person: &Person) -> String {
    serde_json::to_string(person).unwrap()
}

fn deserialize_person(json: &str) -> Person {
    serde_json::from_str(json).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let person = Person {
            name: "Alice".to_string(),
            age: 30,
        };
        let json = serialize_person(&person);
        assert_eq!(json, "{\"name\":\"Alice\",\"age\":30}");
    }

    #[test]
    fn test_deserialize() {
        let json = "{\"name\":\"Bob\",\"age\":25}";
        let person = deserialize_person(json);
        assert_eq!(person, Person {
            name: "Bob".to_string(),
            age: 25,
        });
    }
}
