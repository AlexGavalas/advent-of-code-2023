const EMPTY_CHAR: char = '\0';
const STAR: char = '*';

fn get_item(line: &String, index: i32) -> char {
    if index < 0 {
        return EMPTY_CHAR;
    }

    match line.chars().nth(index as usize) {
        Some(char) => char,
        None => EMPTY_CHAR,
    }
}

fn find_neighbors(line: &String, prev_line: &String, next_line: &String, index: i32) -> Vec<char> {
    let char_above = get_item(prev_line, index);
    let char_below = get_item(next_line, index);
    let char_left = get_item(line, index - 1);
    let char_right = get_item(line, index + 1);
    let char_above_left = get_item(prev_line, index - 1);
    let char_above_right = get_item(prev_line, index + 1);
    let char_below_left = get_item(next_line, index - 1);
    let char_below_right = get_item(next_line, index + 1);

    vec![
        char_above_left,
        char_above,
        char_above_right,
        char_right,
        char_below_right,
        char_below,
        char_below_left,
        char_left,
    ]
}

fn get_number(line: &String, index: i32) -> u32 {
    let num_left: String = line[0..index as usize]
        .chars()
        .rev()
        .take_while(|char| char.is_numeric())
        .collect::<Vec<char>>()
        .iter()
        .rev()
        .collect();

    let num_right: Vec<char> = line[index as usize..]
        .chars()
        .take_while(|char| char.is_numeric())
        .collect();

    let num = (num_left + String::from_iter(num_right).as_str())
        .parse::<u32>()
        .unwrap_or_default();

    num
}

pub fn solve(lines: &Vec<String>) -> u32 {
    let mut index: i32 = 0;
    let empty = String::new();

    lines.iter().fold(0, |count, line| {
        let prev_line = lines.get((index - 1) as usize).unwrap_or(&empty);
        let next_line = lines.get((index + 1) as usize).unwrap_or(&empty);

        let line_sum = line.char_indices().fold(0, |acc, (char_index, char)| {
            if char != STAR {
                return acc;
            }

            let neighbors = find_neighbors(line, prev_line, next_line, char_index as i32);

            let has_adjustent_number = neighbors
                .iter()
                .filter(|char| char.is_alphanumeric())
                .count()
                > 0;

            if has_adjustent_number {
                let num_above_left = get_number(prev_line, char_index as i32);
                let num_above_right = get_number(prev_line, char_index as i32 + 1);
                let num_left = get_number(line, char_index as i32);
                let num_right = get_number(line, char_index as i32 + 1);
                let num_below_left = get_number(next_line, char_index as i32);
                let num_below_right = get_number(next_line, char_index as i32 + 1);

                let nums_above = if num_above_left == num_above_right {
                    vec![num_above_left]
                } else {
                    vec![num_above_left, num_above_right]
                };

                let nums_below = if num_below_left == num_below_right {
                    vec![num_below_left]
                } else {
                    vec![num_below_left, num_below_right]
                };

                let nums_adjacent = vec![num_left, num_right];

                let nums_around = nums_above
                    .iter()
                    .chain(nums_below.iter())
                    .chain(nums_adjacent.iter());

                let nums = nums_around
                    .filter(|num| **num > 0)
                    .map(|num| *num)
                    .collect::<Vec<u32>>();

                let line_sum = match nums.len() {
                    2 => nums[0] * nums[1],
                    _ => 0,
                };

                return acc + line_sum;
            }

            acc
        });

        index += 1;

        count + line_sum
    })
}
