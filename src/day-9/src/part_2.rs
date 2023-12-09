pub fn solve(lines: &Vec<String>) -> i64 {
    lines
        .iter()
        .fold(Vec::<i64>::new(), |mut acc, line| {
            let mut nums = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .rev()
                .collect::<Vec<_>>();

            let mut last_nums: Vec<i64> = vec![nums.last().unwrap().to_owned()];

            loop {
                let part_1 = nums[0..nums.len() - 1].to_vec();
                let part_2 = nums[1..nums.len()].to_vec();

                nums =
                    part_1
                        .iter()
                        .zip(part_2.iter())
                        .fold(Vec::<i64>::new(), |mut acc, (a, b)| {
                            acc.push(b - a);
                            acc
                        });

                if nums.iter().all(|&num| num == 0) {
                    break;
                }

                last_nums.push(nums.last().unwrap().to_owned());
            }

            acc.push(last_nums.iter().sum());
            acc
        })
        .iter()
        .sum()
}
