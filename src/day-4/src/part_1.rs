pub fn solve(lines: &Vec<String>) -> u32 {
    lines.iter().fold(0, |acc, line| {
        let parts = line
            .split_terminator(&[':', '|'])
            .skip(1)
            .collect::<Vec<&str>>();

        let winning_nums = parts[0]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let player_nums = parts[1]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .fold(0 as u32, |acc, num| {
                acc + if winning_nums.contains(&num) { 1 } else { 0 }
            });

        acc + if player_nums != 0 {
            2u32.pow(player_nums - 1)
        } else {
            0
        }
    })
}
