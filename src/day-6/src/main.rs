mod part_1;
mod part_2;

use utils::read_lines;

fn get_input_file_path() -> &'static str {
    "./src/day-6/src/input.txt"
}

fn main() {
    println!("Hello, day 6!");

    let lines = read_lines(get_input_file_path());

    println!("Part 1: (3317888) {}", part_1::solve(&lines));
    println!("Part 2: (24655068) {}", part_2::solve(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    // `cargo test` and `cargo run` have different working directories
    // https://doc.rust-lang.org/cargo/commands/cargo-test.html#working-directory-of-tests
    fn get_test_input_file_path() -> &'static str {
        "./src/input.txt"
    }

    #[test]
    fn test_part_1() {
        let lines = read_lines(get_test_input_file_path());

        assert_eq!(3317888, part_1::solve(&lines));
    }

    #[test]
    fn test_part_2() {
        let lines = read_lines(get_test_input_file_path());

        assert_eq!(24655068, part_2::solve(&lines));
    }

    #[test]
    fn test_get_input_file_path() {
        assert_eq!("./src/day-6/src/input.txt", get_input_file_path());
    }
}
