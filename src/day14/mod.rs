use std::str::FromStr;

struct Polymer {
    elements: Vec<char>,
}

impl Polymer {
    fn apply(self, rules: &[PairInsertion]) -> Self {
        let mut result: Vec<(char, char, Vec<char>)> = self.elements.windows(2).map(|&[a, b]| {
            (a, b, Vec::new())
        }).collect();
        for rule in rules {
            result.iter_mut().find(|&&mut (a, b, _)| a == rule.a && b == rule.b)
                .map(|&mut (_, _, ref mut c)| c.push(rule.c));
        }
        result.into_iter().flat_map(|(a, _, insertions)| ).collect()
    }
}

struct Manual {
    polymer: Polymer,
    rules: Vec<PairInsertion>,
}

struct PairInsertion {
    a: char,
    b: char,
    insertion: char,
}

impl FromStr for Polymer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elements = s.chars().collect();
        Ok(Polymer { elements })
    }
}

impl FromStr for Manual {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (polymer, rules) = input.split_once("\n\n").unwrap();
        Ok(Self {
            polymer: polymer.parse().unwrap(),
            rules: rules.lines().map(|line| line.parse().unwrap()).collect(),
        })
    }
}

impl FromStr for PairInsertion {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (pair, insertion) = input.split_once(" ->").unwrap();
        let mut pair = pair.chars();
        Ok(Self {
            a: pair.next().unwrap(),
            b: pair.next().unwrap(),
            insertion: insertion.chars().next().unwrap(),
        })
    }
}

pub fn part_1(mut paper: Paper) -> usize {
    paper.do_first_instruction();
    paper.number_of_dots()
}

pub fn part_2(mut paper: Paper) -> String {
    paper.do_instructions();
    let paper = paper.to_string();
    println!("{}", paper);
    paper
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    use super::*;

    const EXAMPLE: &str = "\
        6,10\n\
        0,14\n\
        9,10\n\
        0,3\n\
        10,4\n\
        4,11\n\
        6,0\n\
        6,12\n\
        4,1\n\
        0,13\n\
        10,12\n\
        3,4\n\
        3,0\n\
        8,4\n\
        1,10\n\
        2,14\n\
        8,10\n\
        9,0\n\
        \n\
        fold along y=7\n\
        fold along x=5\
    ";

    #[test]
    fn example_1_produces_17() {
        let paper = EXAMPLE.parse().unwrap();
        assert_eq!(17, part_1(paper));
    }

    #[test]
    fn part_1_works() {
        let paper = parse_file("src/day13/input.txt");
        assert_eq!(701, part_1(paper));
    }

    #[test]
    fn example_2_produces_a_square() {
        let paper = EXAMPLE.parse().unwrap();
        assert_eq!(
            "\
                #####\n\
                #...#\n\
                #...#\n\
                #...#\n\
                #####\n\
                .....\n\
                .....\n\
            ",
            part_2(paper)
        );
    }

    #[test]
    fn part_2_works() {
        let paper = parse_file("src/day13/input.txt");
        assert_eq!(
            "\
                ####.###..####.#..#.###..####...##.#....\n\
                #....#..#.#....#.#..#..#.#.......#.#....\n\
                ###..#..#.###..##...###..###.....#.#....\n\
                #....###..#....#.#..#..#.#.......#.#....\n\
                #....#....#....#.#..#..#.#....#..#.#....\n\
                #....#....####.#..#.###..####..##..####.\n\
            ",
            part_2(paper)
        );
    }
}
