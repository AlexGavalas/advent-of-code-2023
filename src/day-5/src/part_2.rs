fn to_number(string_num: &str) -> u64 {
    string_num.parse::<u64>().unwrap()
}

pub fn solve(lines: &Vec<String>) -> u64 {
    let input = lines.join("\n");
    let sections = input.split_terminator("\n\n").collect::<Vec<&str>>();

    let result = sections.iter().fold(Vec::new(), |mut acc, section| {
        let parts = section.split_terminator(':').collect::<Vec<&str>>();
        let section_id = parts[0];

        match section_id {
            "seeds" => parts[1]
                .split_terminator('\n')
                .flat_map(|line| line.split_whitespace())
                .map(to_number)
                .collect::<Vec<_>>()
                .chunks(2)
                .flat_map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
                .collect::<Vec<_>>(),
            _ => {
                let mut destination_maps = parts[1]
                    .split_terminator('\n')
                    .filter(|part| part.len() > 0)
                    .map(|line| line.split_whitespace().map(to_number).collect::<Vec<_>>())
                    .collect::<Vec<_>>();

                if section_id == "seed-to-soil map" {
                    acc.sort_by(|a, b| b.cmp(&a));
                }

                destination_maps.sort_by(|a, b| b[1].cmp(&a[1]));

                acc.iter()
                    .map(|&current_num| {
                        destination_maps
                            .iter()
                            .filter(|destination_map| current_num >= destination_map[1])
                            .find_map(|destination_map| {
                                let destination_range_start = destination_map[0];
                                let source_range_start = destination_map[1];
                                let range_length = destination_map[2];

                                if current_num >= source_range_start
                                    && current_num <= (source_range_start + range_length)
                                {
                                    let diff = current_num - source_range_start;

                                    return Some(destination_range_start + diff);
                                }

                                if current_num > (source_range_start + range_length) {
                                    return Some(current_num);
                                }

                                None
                            })
                            .unwrap_or(current_num)
                    })
                    .collect()
            }
        }
    });

    *result.iter().min().unwrap()
}
