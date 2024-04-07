pub fn format_map_entry(map_entry: &str) -> (&str, &str, &str) {
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

pub fn get_next_direction(directions: &Vec<char>, directions_index: usize) -> (char, usize) {
    let mut index = directions_index;

    if index == directions.len() {
        index = 0;
    }

    let direction = directions[index];

    (direction, index + 1)
}
