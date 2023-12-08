use std::collections::HashMap;

fn format_map_entry(map_entry: &str) -> (&str, &str, &str) {
    let parts = map_entry
        .split_terminator(&['=', ',', '(', ')'])
        .filter_map(|v| {
            let trimmed = v.trim();

            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
        .collect::<Vec<_>>();

    (parts[0], parts[1], parts[2])
}

fn get_next_direction(directions: &Vec<char>, directions_index: usize) -> (char, usize) {
    let mut index = directions_index;

    if index == directions.len() {
        index = 0;
    }

    let direction = directions[index];

    (direction, index + 1)
}

pub fn solve(lines: &Vec<String>) -> u64 {
    let parts = lines.split_at(1);

    let str_directions = parts.0[0].to_string();
    let directions = str_directions.chars().collect::<Vec<_>>();

    let map = parts.1[1..].to_vec();

    let hash_map = map.iter().fold(HashMap::new(), |mut acc, v| {
        let (position, left, right) = format_map_entry(v);

        acc.insert(position, (position, left, right));
        acc
    });

    let mut positions = map
        .iter()
        .filter_map(|v| {
            let map_entry = format_map_entry(v);

            if map_entry.0.ends_with("A") {
                Some(map_entry)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut steps = 0;
    let mut directions_index = 0;

    loop {
        steps += 1;

        let (direction, new_index) = get_next_direction(&directions, directions_index);
        directions_index = new_index;

        positions = positions
            .iter()
            .map(|(_, left, right)| match direction {
                'R' => hash_map.get(right).unwrap().to_owned(),
                'L' => hash_map.get(left).unwrap().to_owned(),
                _ => panic!("Unknown direction: {}", direction),
            })
            .collect::<Vec<_>>();

        if !positions.iter().any(|v| !v.0.ends_with("Z")) {
            break steps;
        }
    }
}
