use crate::helpers::{format_map_entry, get_next_direction};
use std::collections::HashMap;

fn move_n_steps<'a>(
    n: u64,
    init_directions_index: usize,
    directions: &'a Vec<char>,
    hash_map: &'a HashMap<&'a str, (&'a str, &'a str, &'a str)>,
    init_positions: Vec<(&'a str, &'a str, &'a str)>,
) -> (usize, Vec<(&'a str, &'a str, &'a str)>) {
    let mut positions = init_positions;
    let mut directions_index = init_directions_index;

    for _ in 0..n {
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
    }

    (directions_index, positions)
}

#[derive(Debug)]
struct Finish<'a> {
    steps: u64,
    directions_index: usize,
    positions: Vec<(&'a str, &'a str, &'a str)>,
}

fn find_next_finish<'a>(
    init_directions_index: usize,
    directions: &'a Vec<char>,
    hash_map: &'a HashMap<&'a str, (&'a str, &'a str, &'a str)>,
    init_positions: Vec<(&'a str, &'a str, &'a str)>,
) -> Finish<'a> {
    let mut inner_steps = 0;
    let mut inner_positions = init_positions;
    let mut inner_dir_index = init_directions_index;

    loop {
        inner_steps += 1;

        let (new_index, new_positions) =
            move_n_steps(1, inner_dir_index, &directions, &hash_map, inner_positions);

        inner_dir_index = new_index;
        inner_positions = new_positions;

        if !inner_positions.iter().any(|v| !v.0.ends_with("Z")) {
            break Finish {
                steps: inner_steps,
                directions_index: inner_dir_index,
                positions: inner_positions,
            };
        }
    }
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
        let mut finishes = positions
            .iter()
            .map(|&position| {
                find_next_finish(directions_index, &directions, &hash_map, vec![position])
            })
            .collect::<Vec<_>>();

        finishes.sort_by(|a, b| b.steps.cmp(&a.steps));

        let c_steps = finishes.iter().map(|v| v.steps).collect::<Vec<_>>();

        steps += c_steps[0];

        directions_index = finishes[0].directions_index;

        positions = finishes
            .iter()
            .flat_map(|v| v.positions.to_owned())
            .collect::<Vec<_>>();

        // NOT 23147

        c_steps
            .iter()
            .map(|s| (c_steps[0] - s).rem_euclid(directions.len() as u64))
            .collect::<Vec<_>>()
            .iter()
            .enumerate()
            .for_each(|(i, &v)| {
                if i != 0 {
                    let (_new_index, new_positions) = move_n_steps(
                        v,
                        finishes[i].directions_index,
                        &directions,
                        &hash_map,
                        vec![positions[i]],
                    );

                    positions[i] = new_positions[0];
                } else {
                    positions[0] = finishes[0].positions[0];
                }
            });

        if !positions.iter().any(|v| !v.0.ends_with("Z")) {
            break steps;
        }
    }
}
