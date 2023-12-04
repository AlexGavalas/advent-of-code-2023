pub fn solve(lines: &Vec<String>) -> u32 {
    let mut copies = vec![1; lines.len()];

    lines.iter().fold(0, |index, line| {
        let parts = line
            .split_terminator(&[':', '|'])
            .skip(1)
            .collect::<Vec<&str>>();

        let winning_nums = parts[0].split_whitespace().collect::<Vec<&str>>();

        let player_nums = parts[1].split_whitespace().fold(0 as u32, |acc, num| {
            acc + if winning_nums.contains(&num) { 1 } else { 0 }
        });

        let card_copies = copies[index as usize];

        for win in 1..player_nums + 1 {
            copies[(index + win) as usize] += card_copies;
        }

        index + 1
    });

    copies.iter().sum::<u32>()
}
