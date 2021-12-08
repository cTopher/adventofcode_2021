pub use command::Command;
use position::NaivePosition;

use crate::day02::position::FullPosition;

mod command;
mod position;

pub fn part_1<I: Iterator<Item = Command>>(instructions: I) -> u32 {
    let NaivePosition { horizontal, depth } =
        instructions.fold(NaivePosition::default(), NaivePosition::apply);
    horizontal * depth
}

pub fn part_2<I: Iterator<Item = Command>>(instructions: I) -> u32 {
    let FullPosition {
        horizontal, depth, ..
    } = instructions.fold(FullPosition::default(), FullPosition::apply);
    horizontal * depth
}

#[cfg(test)]
mod tests {
    use crate::parse_file_lines;
    use crate::parse_str_lines;

    use super::*;

    const EXAMPLE: &str = "\
        forward 5\n\
        down 5\n\
        forward 8\n\
        up 3\n\
        down 8\n\
        forward 2\
    ";

    #[test]
    fn example_1_produces_150() {
        let input = parse_str_lines(EXAMPLE);
        let result = part_1(input);
        assert_eq!(result, 150);
    }

    #[test]
    fn part_1_works() {
        let input = parse_file_lines("src/day02/input.txt");
        let result = part_1(input);
        assert_eq!(result, 1_507_611);
    }

    #[test]
    fn example_2_produces_900() {
        let input = parse_str_lines(EXAMPLE);
        let result = part_2(input);
        assert_eq!(result, 900);
    }

    #[test]
    fn part_2_works() {
        let input = parse_file_lines("src/day02/input.txt");
        let result = part_2(input);
        assert_eq!(result, 1_880_593_125);
    }
}
