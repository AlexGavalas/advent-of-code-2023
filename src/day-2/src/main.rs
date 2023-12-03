mod part_1;
mod part_2;

use utils::read_lines;

fn main() {
    println!("Hello, day 2!");

    let lines = read_lines("./src/day-2/src/input.txt");

    println!("Part 1: (2545) {}", part_1::solve(&lines));
    println!("Part 2: (78111) {}", part_2::solve(&lines));
}
