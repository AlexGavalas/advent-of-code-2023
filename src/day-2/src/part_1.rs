pub fn solve(lines: &Vec<String>) -> u32 {
    lines.iter().fold(0, |count, line| {
        let parts: Vec<&str> = line
            .split_terminator(&[':', ';'][..])
            .map(|part| part.trim())
            .collect();

        let game_id = parts[0]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let reveals = &parts[1..];

        let is_possible = reveals.iter().all(|reveal| {
            let parts: Vec<&str> = reveal
                .split_terminator(',')
                .map(|part| part.trim())
                .collect();

            parts.iter().all(|part| {
                let result: Vec<&str> = part.split_whitespace().collect();
                let count = result[0].parse::<u32>().unwrap();
                let color = result[1];

                match color {
                    "red" => count <= 12,
                    "green" => count <= 13,
                    "blue" => count <= 14,
                    _ => false,
                }
            })
        });

        count + if is_possible { game_id } else { 0 }
    })
}
