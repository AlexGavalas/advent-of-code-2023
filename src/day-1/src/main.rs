mod part_1;
mod part_2;

use utils::read_lines;

fn main() {
    println!("Hello, day 1!");

    let lines = read_lines("./src/day-1/src/input.txt");

    println!("Part 1: (53194) {}", part_1::solve(&lines));
    println!("Part 2: (54249) {}", part_2::solve(&lines));
}
