fn scope_and_shadowing() -> (i32, i32, String, String) {
    let mut x = 5;
    let y = &mut x;
    let mut s1 = String::from("outer");

    {
        let x = 10;
        let z = &x;

        *y = *y * 5;
        let s2 = String::from("inner");

        s1.push_str("-modified");

        return (*y, *z, s1, s2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_and_shadowing() {
        assert_eq!(
            scope_and_shadowing(),
            (
                25,
                10,
                String::from("outer-modified"),
                String::from("inner")
            )
        );
    }
}
