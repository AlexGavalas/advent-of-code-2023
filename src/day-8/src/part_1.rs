fn format_map_entry(map_entry: &str) -> (&str, &str, &str) {
    let parts = map_entry
        .split_terminator(&['=', ',', '(', ')'])
        .filter(|&v| v.trim().len() > 0)
        .map(|v| v.trim())
        .collect::<Vec<_>>();

    (parts[0], parts[1], parts[2])
}

pub fn solve(lines: &Vec<String>) -> u64 {
    let parts = lines.split_at(1);

    let str_directions = parts.0[0].to_string();
    let directions_iter = str_directions.chars().rev().collect::<Vec<_>>();
    let mut directions = directions_iter;

    let map = parts.1[1..].to_vec();
    let mut position = map.iter().find(|&v| v.starts_with("AAA")).unwrap().as_str();

    let mut steps = 0;

    loop {
        steps += 1;

        let mut direction = directions.pop().unwrap_or_default();

        if !direction.is_alphabetic() {
            directions = str_directions.chars().rev().collect::<Vec<_>>();
            direction = directions.pop().unwrap();
        }

        let (_, left, right) = format_map_entry(position);

        match direction {
            'R' => {
                if right == "ZZZ" {
                    break steps;
                }

                position = map.iter().find(|&v| v.starts_with(right)).unwrap();
            }
            'L' => {
                if left == "ZZZ" {
                    break steps;
                }

                position = map.iter().find(|&v| v.starts_with(left)).unwrap();
            }
            _ => {}
        };
    }
}
