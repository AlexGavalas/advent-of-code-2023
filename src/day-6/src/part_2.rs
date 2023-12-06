fn parse_line(line: &String) -> u128 {
    let content = line.split_terminator(':').skip(1).collect::<Vec<_>>();

    let parts = content[0].split_whitespace().collect::<Vec<_>>().join("");

    parts.parse::<u128>().unwrap()
}

pub fn solve(lines: &Vec<String>) -> u128 {
    let time = parse_line(&lines[0]);
    let distance = parse_line(&lines[1]);

    (0..time).fold(0, |wins, initial_speed| {
        let remaining_time = time - initial_speed;

        if remaining_time * initial_speed > distance {
            return wins + 1;
        }

        wins
    })
}
