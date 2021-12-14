use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Default, Clone, Debug)]
struct Polymer {
    elements: Vec<char>,
}

impl Polymer {
    fn apply_n(&self, rules: &[PairInsertion], n: usize) -> Self {
        (1..n).fold(self.apply(rules), |polymer, _| polymer.apply(rules))
    }

    fn apply(&self, rules: &[PairInsertion]) -> Self {
        let mut result: Vec<(Vec<char>, char)> = self
            .elements
            .windows(2)
            .map(|window| (vec![window[0]], window[1]))
            .collect();
        for rule in rules {
            result
                .iter_mut()
                .filter(|(chars, b)| chars[0] == rule.a && b == &rule.b)
                .for_each(|(chars, _)| chars.push(rule.insertion));
        }
        let elements: Vec<char> = result
            .into_iter()
            .flat_map(|(chars, _)| chars)
            .chain(std::iter::once(*self.elements.last().unwrap()))
            .collect();
        Self { elements }
    }

    pub fn counts(&self) -> HashMap<char, u64> {
        let mut counts: HashMap<char, u64> = HashMap::new();
        for c in self.elements.iter() {
            *counts.entry(*c).or_insert(0) += 1;
        }
        counts
    }
}

impl fmt::Display for Polymer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_iter(self.elements.iter()))
    }
}

#[derive(Default, Clone, Debug)]
pub struct Manual {
    polymer: Polymer,
    rules: Vec<PairInsertion>,
}

#[derive(Clone, Debug)]
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
        let (pair, insertion) = input.split_once(" -> ").unwrap();
        let mut pair = pair.chars();
        Ok(Self {
            a: pair.next().unwrap(),
            b: pair.next().unwrap(),
            insertion: insertion.chars().next().unwrap(),
        })
    }
}

pub fn part_1(manual: &Manual) -> u64 {
    let counts = manual.polymer.apply_n(&manual.rules, 10).counts();
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

pub fn part_2(manual: &Manual) -> u64 {
    let counts = manual.polymer.apply_n(&manual.rules, 40).counts();
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    use super::*;

    const EXAMPLE: &str = "\
        NNCB\n\
        \n\
        CH -> B\n\
        HH -> N\n\
        CB -> H\n\
        NH -> C\n\
        HB -> C\n\
        HC -> B\n\
        HN -> C\n\
        NN -> C\n\
        BH -> H\n\
        NC -> B\n\
        NB -> B\n\
        BN -> B\n\
        BB -> N\n\
        BC -> B\n\
        CC -> N\n\
        CN -> C\n\
    ";

    #[test]
    fn example_1_produces_1588() {
        let manual = EXAMPLE.parse().unwrap();
        assert_eq!(1588, part_1(&manual));
    }

    #[test]
    fn part_1_works() {
        let manual = parse_file("src/day14/input.txt");
        assert_eq!(3411, part_1(&manual));
    }

    // #[test]
    // fn example_2_produces_2188189693529() {
    //     let manual = EXAMPLE.parse().unwrap();
    //     assert_eq!(2188189693529, part_2(&manual));
    // }
    //
    // #[test]
    // fn part_2_works() {
    //     let manual = parse_file("src/day14/input.txt");
    //     assert_eq!(3411, part_1(&manual));
    // }
}
