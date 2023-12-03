mod part_1;
mod part_2;

use utils::read_lines;

fn main() {
    println!("Hello, day 3!");

    let lines = read_lines("./src/day-3/src/input.txt");

    println!("Part 1: (514969) {}", part_1::solve(&lines));
    println!("Part 2: (78915902) {}", part_2::solve(&lines));
}
