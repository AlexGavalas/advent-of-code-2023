pub fn solve(lines: &Vec<String>) -> u32 {
    let string_nums = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    lines.iter().fold(0, |acc, line| {
        let first_char = line
            .chars()
            .fold("".to_string(), |acc, c| {
                if string_nums.contains(&acc.as_str()) {
                    return acc;
                }

                if c.is_numeric() {
                    return c.to_string();
                }

                let new_value = acc + &c.to_string();

                let maybe_num = match new_value.as_str() {
                    v if v.ends_with("zero") => 0,
                    v if v.ends_with("one") => 1,
                    v if v.ends_with("two") => 2,
                    v if v.ends_with("three") => 3,
                    v if v.ends_with("four") => 4,
                    v if v.ends_with("five") => 5,
                    v if v.ends_with("six") => 6,
                    v if v.ends_with("seven") => 7,
                    v if v.ends_with("eight") => 8,
                    v if v.ends_with("nine") => 9,
                    _ => -1,
                };

                if maybe_num != -1 {
                    return maybe_num.to_string();
                }

                new_value
            })
            .parse::<u32>()
            .unwrap();

        let last_char = line
            .chars()
            .rfold("".to_string(), |acc, c| {
                if string_nums.contains(&acc.as_str()) {
                    return acc;
                }

                if c.is_numeric() {
                    return c.to_string();
                }

                let new_value = acc + &c.to_string();

                let maybe_num = match new_value.as_str() {
                    v if v.ends_with("orez") => 0,
                    v if v.ends_with("eno") => 1,
                    v if v.ends_with("owt") => 2,
                    v if v.ends_with("eerht") => 3,
                    v if v.ends_with("ruof") => 4,
                    v if v.ends_with("evif") => 5,
                    v if v.ends_with("xis") => 6,
                    v if v.ends_with("neves") => 7,
                    v if v.ends_with("thgie") => 8,
                    v if v.ends_with("enin") => 9,
                    _ => -1,
                };

                if maybe_num != -1 {
                    return maybe_num.to_string();
                }

                new_value
            })
            .parse::<u32>()
            .unwrap_or(first_char);

        acc + first_char * 10 + last_char
    })
}
