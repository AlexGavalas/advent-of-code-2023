use crate::helpers::{get_char, get_starting_direction, get_starting_position, Compass};

pub fn solve(lines: &Vec<String>) -> u32 {
    let starting_position = get_starting_position(lines);
    let (mut next_position, mut compass) = get_starting_direction(lines, starting_position);
    let mut steps = 0;

    let total_steps: u32 = loop {
        steps += 1;

        let current_tile = get_char(lines, next_position);

        match (current_tile, compass) {
            ('|', Compass::North) => next_position.y -= 1,
            ('|', Compass::South) => next_position.y += 1,
            ('-', Compass::East) => next_position.x += 1,
            ('-', Compass::West) => next_position.x -= 1,
            ('L', Compass::South) => {
                next_position.x += 1;
                compass = Compass::East;
            }
            ('L', Compass::West) => {
                next_position.y -= 1;
                compass = Compass::North;
            }
            ('J', Compass::South) => {
                next_position.x -= 1;
                compass = Compass::West;
            }
            ('J', Compass::East) => {
                next_position.y -= 1;
                compass = Compass::North;
            }
            ('7', Compass::North) => {
                next_position.x -= 1;
                compass = Compass::West;
            }
            ('7', Compass::East) => {
                next_position.y += 1;
                compass = Compass::South;
            }
            ('F', Compass::North) => {
                next_position.x += 1;
                compass = Compass::East;
            }
            ('F', Compass::West) => {
                next_position.y += 1;
                compass = Compass::South;
            }
            (_, _) => panic!("Invalid tile {}", current_tile),
        };

        let next_tile = get_char(lines, next_position);

        if next_tile == 'S' {
            break steps;
        }
    };

    total_steps / 2 + total_steps % 2
}
