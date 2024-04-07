#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Compass {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

pub fn get_char(str: &Vec<String>, coordinate: Coordinate) -> char {
    str[coordinate.y].chars().nth(coordinate.x).unwrap()
}

pub fn get_starting_position(lines: &Vec<String>) -> Coordinate {
    lines
        .iter()
        .enumerate()
        .find_map(|(line_index, line)| {
            let char_index = line.char_indices().position(|(_, char)| char == 'S');

            if let Some(x_index) = char_index {
                Some(Coordinate {
                    x: x_index,
                    y: line_index,
                })
            } else {
                None
            }
        })
        .unwrap()
}

pub fn get_starting_direction(
    lines: &Vec<String>,
    starting_position: Coordinate,
) -> (Coordinate, Compass) {
    let mut next_position = starting_position;

    let bottom_tile = get_char(
        lines,
        Coordinate {
            x: starting_position.x,
            y: starting_position.y + 1,
        },
    );

    let right_tile = get_char(
        lines,
        Coordinate {
            x: starting_position.x + 1,
            y: starting_position.y,
        },
    );

    let left_tile = if starting_position.x != 0 {
        get_char(
            lines,
            Coordinate {
                x: starting_position.x - 1,
                y: starting_position.y,
            },
        )
    } else {
        ' '
    };

    let top_tile = if starting_position.y != 0 {
        get_char(
            lines,
            Coordinate {
                x: starting_position.x,
                y: starting_position.y - 1,
            },
        )
    } else {
        ' '
    };

    let compass = match (top_tile, right_tile, bottom_tile, left_tile) {
        ('|' | 'F' | '7', _, _, _) => {
            next_position.y -= 1;
            Compass::North
        }
        (_, '-' | 'J' | '7', _, _) => {
            next_position.x += 1;
            Compass::East
        }
        (_, _, '|' | 'J' | 'L', _) => {
            next_position.y += 1;
            Compass::South
        }
        (_, _, _, '-' | 'F' | 'L') => {
            next_position.x -= 1;
            Compass::West
        }
        _ => panic!(
            "Invalid starting tile: {}{}{}{}",
            bottom_tile, right_tile, left_tile, top_tile
        ),
    };

    (next_position, compass)
}
