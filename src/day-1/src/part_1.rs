pub fn solve(lines: &Vec<String>) -> u32 {
    lines.iter().fold(0, |acc, line| {
        let first_char = line
            .chars()
            .find(|c| c.is_numeric())
            .unwrap()
            .to_digit(10)
            .unwrap();

        let last_char = line
            .chars()
            .rfind(|c| c.is_numeric())
            .unwrap()
            .to_digit(10)
            .unwrap_or(first_char);

        acc + first_char * 10 + last_char
    })
}
