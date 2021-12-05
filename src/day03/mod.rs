use std::ops::Add;

use bits::Bits;
use count::BitCounts;

mod bits;
mod count;

pub fn power_consumption<I: Iterator<Item = Bits>>(mut report: I) -> u32 {
    let init = BitCounts::from(report.next().unwrap());
    let BitCounts { counts, size } = report.fold(init, BitCounts::add);
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for (index, count) in counts.into_iter().rev().enumerate() {
        if count > size / 2 {
            gamma_rate |= 1 << index;
        } else {
            epsilon_rate |= 1 << index;
        }
    }
    gamma_rate * epsilon_rate
}

pub fn life_support_rating(report: Vec<Bits>) -> u32 {
    oxygen_generator_rating(report.clone()) * co2_scrubber_rating(report)
}

fn oxygen_generator_rating(report: Vec<Bits>) -> u32 {
    rating(report, true)
}

fn co2_scrubber_rating(report: Vec<Bits>) -> u32 {
    rating(report, false)
}

fn rating(mut report: Vec<Bits>, bit_criteria: bool) -> u32 {
    for position in 0..(report[0].len()) {
        let count = report.iter().filter(|bits| bits[position]).count();
        let criteria = xnor(2 * count >= report.len(), bit_criteria);
        report.retain(|bits| bits[position] == criteria);
        if report.len() == 1 {
            return report[0].as_u32();
        }
    }
    panic!("invalid diagnostic report")
}

const fn xnor(a: bool, b: bool) -> bool {
    !(a ^ b)
}

#[cfg(test)]
mod tests {
    use crate::parse_file_lines;
    use crate::parse_str_lines;

    use super::*;

    const EXAMPLE: &str = "\
        00100\n\
        11110\n\
        10110\n\
        10111\n\
        10101\n\
        01111\n\
        00111\n\
        11100\n\
        10000\n\
        11001\n\
        00010\n\
        01010";

    #[test]
    fn example_power_consumption_is_198() {
        let input = parse_str_lines(EXAMPLE);
        let result = power_consumption(input);
        assert_eq!(result, 198);
    }

    #[test]
    fn part_1_works() {
        let input = parse_file_lines("src/day03/input.txt");
        let result = power_consumption(input);
        assert_eq!(result, 4_191_876);
    }

    #[test]
    fn example_life_support_rating_is_230() {
        let input = parse_str_lines(EXAMPLE).collect();
        let result = life_support_rating(input);
        assert_eq!(result, 230);
    }

    #[test]
    fn part_2_works() {
        let input = parse_file_lines("src/day03/input.txt").collect();
        let result = life_support_rating(input);
        assert_eq!(result, 3_414_905);
    }
}
