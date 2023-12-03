const EMPTY_CHAR: char = '\0';
const DOT: char = '.';

fn is_symbol(char: &&char) -> bool {
    !char.is_alphanumeric() && *char != &DOT && *char != &EMPTY_CHAR
}

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

pub fn solve(lines: &Vec<String>) -> u32 {
    let mut index: i32 = 0;
    let empty = String::new();

    lines.iter().fold(0, |count, line| {
        let prev_line = lines.get((index - 1) as usize).unwrap_or(&empty);
        let next_line = lines.get((index + 1) as usize).unwrap_or(&empty);

        let (_, nums) =
            line.char_indices()
                .fold((Vec::new(), Vec::new()), |(acc, result), (index, char)| {
                    if !char.is_alphanumeric() {
                        return (acc, result);
                    }

                    let neighbors = find_neighbors(line, prev_line, next_line, index as i32);

                    let has_adjustent_symbol = neighbors.iter().filter(is_symbol).count() > 0;

                    if has_adjustent_symbol {
                        let mut acc_cloned: Vec<u32> = acc.clone();
                        acc_cloned.push(index as u32);

                        let last_element = match acc.last() {
                            Some(element) => element,
                            None => &0,
                        };

                        if index as u32 - last_element == 1 {
                            return (acc_cloned, result);
                        }

                        let num_left: String = line[0..index]
                            .chars()
                            .rev()
                            .take_while(|char| char.is_numeric())
                            .collect::<Vec<char>>()
                            .iter()
                            .rev()
                            .collect();

                        let num_right: Vec<char> = line[index..]
                            .chars()
                            .take_while(|char| char.is_numeric())
                            .collect();

                        let num = (num_left + String::from_iter(num_right).as_str())
                            .parse::<u32>()
                            .unwrap();

                        let mut result_cloned: Vec<u32> = result.clone();
                        result_cloned.push(num);

                        return (acc_cloned, result_cloned);
                    }

                    (acc, result)
                });

        let line_sum: u32 = nums.iter().sum();

        index += 1;

        count + line_sum
    })
}
