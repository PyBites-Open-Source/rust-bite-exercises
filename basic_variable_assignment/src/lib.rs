fn increment_by_one(x: i32) -> i32 {
    x + 1 // Copies the value held by x and then adds 1 to it
}

fn increment_mutable(x: &mut i32) -> () {
    *x += 1; // Dereference and increment the value held by x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_by_one() {
        assert_eq!(increment_by_one(0), 1);
        assert_eq!(increment_by_one(-20), -19);
        assert_eq!(increment_by_one(5), 6);
    }

    #[test]
    fn test_increment_mutable() {
        let mut value = 0;
        increment_mutable(&mut value);
        assert_eq!(value, 1);

        let mut value = -20;
        increment_mutable(&mut value);
        assert_eq!(value, -19);

        let mut value = 5;
        increment_mutable(&mut value);
        assert_eq!(value, 6);
    }
}
