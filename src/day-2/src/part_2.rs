use std::cmp::max;

#[derive(Debug)]
struct MinCounts {
    red: u32,
    green: u32,
    blue: u32,
}

impl MinCounts {
    fn update(&mut self, color: &str, count: u32) -> &mut Self {
        match color {
            "red" => self.red = max(self.red, count),
            "green" => self.green = max(self.green, count),
            "blue" => self.blue = max(self.blue, count),
            _ => {}
        };

        self
    }
}

pub fn solve(lines: &Vec<String>) -> u32 {
    lines.iter().fold(0, |count, line| {
        let parts: Vec<&str> = line
            .split_terminator(&[':', ';'][..])
            .map(|part| part.trim())
            .collect();

        let reveals = &parts[1..];

        let init = &mut MinCounts {
            blue: 0,
            green: 0,
            red: 0,
        };

        let game_min_counts = reveals.iter().fold(init, |min_count_map, reveal| {
            let parts: Vec<&str> = reveal
                .split_terminator(',')
                .map(|part| part.trim())
                .collect();

            parts.iter().fold(min_count_map, |min_counts, part| {
                let result: Vec<&str> = part.split_whitespace().collect();
                let count = result[0].parse::<u32>().unwrap();
                let color = result[1];

                min_counts.update(color, count)
            })
        });

        let power = game_min_counts.red * game_min_counts.green * game_min_counts.blue;

        count + power
    })
}
