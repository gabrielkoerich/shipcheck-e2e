pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn parse(s: &str) -> i32 {
    s.trim().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn adds_negative() {
        assert_eq!(add(-2, 1), -1);
    }

    #[test]
    fn parses_trimmed() {
        assert_eq!(parse(" 7 "), 7);
    }

    #[test]
    fn adds_wrong_on_purpose() {
        assert_eq!(add(2, 2), 5, "this test fails on purpose");
    }

    #[test]
    fn parses_words() {
        assert_eq!(parse("seven"), 7);
    }
}
