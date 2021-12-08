use std::collections::HashSet;
use std::str::FromStr;

#[derive(Default, Clone, Debug, Eq, PartialEq)]
struct Pattern {
    segments: HashSet<char>,
}

impl Pattern {
    fn len(&self) -> usize {
        self.segments.len()
    }

    fn contains(&self, other: &Self) -> bool {
        other.segments.iter().all(|c| self.segments.contains(c))
    }

    fn overlap(&self, other: &Self) -> usize {
        other
            .segments
            .iter()
            .filter(|c| self.segments.contains(c))
            .count()
    }
}

impl FromStr for Pattern {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let segments = input.to_string().chars().collect();
        Ok(Self { segments })
    }
}

pub struct Entry {
    input: Vec<Pattern>,
    output: Vec<Pattern>,
}

impl Entry {
    fn solve(mut self) -> usize {
        let x1 = self.take_len(2);
        let x4 = self.take_len(4);
        let x7 = self.take_len(3);
        let x8 = self.take_len(7);
        let x9 = self.take(|pattern| pattern.len() == 6 && pattern.contains(&x4));
        let x0 = self.take(|pattern| pattern.len() == 6 && pattern.contains(&x1));
        let x6 = self.take_len(6);
        let x3 = self.take(|pattern| pattern.contains(&x1));
        let x2 = self.take(|pattern| pattern.overlap(&x4) == 2);
        let x5 = self.take(|_| true);

        let numbers = [x0, x1, x2, x3, x4, x5, x6, x7, x8, x9];

        self.output
            .iter()
            .map(|pattern| numbers.iter().position(|n| n == pattern).unwrap())
            .reduce(|acc, n| acc * 10 + n)
            .unwrap()
    }

    fn take_len(&mut self, len: usize) -> Pattern {
        self.take(|pattern| pattern.len() == len)
    }

    fn take<P: Fn(&Pattern) -> bool>(&mut self, predicate: P) -> Pattern {
        let index = self.input.iter().position(predicate).unwrap();
        self.input.remove(index)
    }
}

pub fn part_1<I: Iterator<Item = Entry>>(entries: I) -> usize {
    entries
        .flat_map(|entry| entry.output)
        .filter(|pattern| {
            let len = pattern.segments.len();
            len == 2 || len == 4 || len == 3 || len == 7
        })
        .count()
}

pub fn part_2<I: Iterator<Item = Entry>>(entries: I) -> usize {
    entries.map(Entry::solve).sum()
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (input, output) = string.split_once(" | ").unwrap();
        let parse = |patterns: &str| {
            patterns
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        };
        Ok(Self {
            input: parse(input),
            output: parse(output),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_file_lines, parse_str_lines};

    use super::*;

    const EXAMPLE: &str = "\
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\n\
    ";

    #[test]
    fn example_1_produces_26() {
        let entries = parse_str_lines(EXAMPLE);
        assert_eq!(26, part_1(entries));
    }

    #[test]
    fn part_1_works() {
        let entries = parse_file_lines("src/day08/input.txt");
        assert_eq!(330, part_1(entries));
    }

    #[test]
    fn example_2_produces_61229() {
        let entries = parse_str_lines(EXAMPLE);
        assert_eq!(61229, part_2(entries));
    }

    #[test]
    fn part_2_works() {
        let entries = parse_file_lines("src/day08/input.txt");
        assert_eq!(1_010_472, part_2(entries));
    }
}
