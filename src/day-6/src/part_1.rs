fn parse_line(line: &String) -> Vec<u64> {
    line.split_terminator(':')
        .skip(1)
        .collect::<Vec<_>>()
        .iter()
        .flat_map(|x| {
            x.split_whitespace()
                .map(|time| time.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn solve(lines: &Vec<String>) -> u64 {
    let times = parse_line(&lines[0]);
    let distance = parse_line(&lines[1]);

    times
        .iter()
        .zip(distance.iter())
        .map(|(&time, &distance)| {
            (0..time).fold(0, |wins, initial_speed| {
                let remaining_time = time - initial_speed;

                if remaining_time * initial_speed > distance {
                    return wins + 1;
                }

                wins
            })
        })
        .collect::<Vec<u64>>()
        .iter()
        .fold(1, |acc, &x| acc * x)
}
