fn is_number(c: &char) -> bool {
    c.is_numeric()
}

fn find_first_number(line: &str) -> u32 {
    line.chars()
        .find(is_number)
        .unwrap_or_default()
        .to_digit(10)
        .unwrap_or_default()
}

fn find_last_number(line: &str, first_number: u32) -> u32 {
    line.chars()
        .rfind(is_number)
        .unwrap_or_default()
        .to_digit(10)
        .unwrap_or(first_number)
}

pub fn solve(lines: &Vec<String>) -> u32 {
    lines.iter().fold(0, |acc, line| {
        let first_number = find_first_number(line);
        let last_number = find_last_number(line, first_number);

        acc + first_number * 10 + last_number
    })
}

#[cfg(test)]
pub mod part_1_test {
    use super::*;

    #[test]
    fn test_is_number() {
        assert_eq!(is_number(&'1'), true);
    }

    #[test]
    fn test_is_number_non_number() {
        assert_eq!(is_number(&'s'), false);
    }

    #[test]
    fn test_find_first_number() {
        assert_eq!(find_first_number("test1ads2..3"), 1);
    }

    #[test]
    fn test_find_first_number_default() {
        assert_eq!(find_first_number("testtest"), 0);
    }

    #[test]
    fn test_find_last_number() {
        assert_eq!(find_last_number("test1ads2..3", 1), 3);
    }

    #[test]
    fn test_find_last_number_default() {
        assert_eq!(find_last_number("testtest", 1), 1);
    }
}
