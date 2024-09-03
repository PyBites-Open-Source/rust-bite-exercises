use std::collections::HashMap;

fn parse_query_params(query: &str) -> HashMap<String, String> {
    // Implement the function to parse query parameters from the input string
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_param() {
        let query = "name=John";
        let params = parse_query_params(query);
        let mut expected = HashMap::new();
        expected.insert("name".to_string(), "John".to_string());
        assert_eq!(params, expected);
    }

    #[test]
    fn test_multiple_params() {
        let query = "name=John&age=30";
        let params = parse_query_params(query);
        let mut expected = HashMap::new();
        expected.insert("name".to_string(), "John".to_string());
        expected.insert("age".to_string(), "30".to_string());
        assert_eq!(params, expected);
    }

    #[test]
    fn test_empty_query() {
        let query = "";
        let params = parse_query_params(query);
        let expected: HashMap<String, String> = HashMap::new();
        assert_eq!(params, expected);
    }

    #[test]
    fn test_param_without_value() {
        let query = "name=";
        let params = parse_query_params(query);
        let mut expected = HashMap::new();
        expected.insert("name".to_string(), "".to_string());
        assert_eq!(params, expected);
    }
}
