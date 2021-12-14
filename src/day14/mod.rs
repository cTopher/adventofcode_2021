use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Default, Clone, Debug)]
struct Polymer {
    elements: Vec<char>,
}

impl Polymer {
    fn first(&self) -> char {
        self.elements[0]
    }

    fn last(&self) -> char {
        self.elements[self.elements.len() - 1]
    }

    fn apply_n(&self, rules: &[Polymer], n: usize) -> Self {
        (1..n).fold(self.apply(rules), |polymer, _| polymer.apply(rules))
    }

    fn apply(&self, rules: &[Polymer]) -> Self {
        let elements: Vec<char> = self
            .elements
            .windows(2)
            .flat_map(|window| {
                rules.iter().find(|&rule| rule.first() == window[0] && rule.last() == window[1])
                    .map(|rule| rule.elements.iter().copied().take(rule.elements.len()-1).collect())
                    .unwrap_or_else(|| vec![window[0]])
            })
            .chain(std::iter::once(self.last()))
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
    rules: Vec<Polymer>,
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
            rules: rules.lines().map(parse_rule).collect(),
        })
    }
}

fn parse_rule(input: &str) -> Polymer {
    let (pair, insertion) = input.split_once(" -> ").unwrap();
    let mut pair = pair.chars();
    let elements = vec![
        pair.next().unwrap(),
        insertion.chars().next().unwrap(),
        pair.next().unwrap(),
    ];
    Polymer { elements }
}

pub fn part_1(manual: &Manual) -> u64 {
    let r1: Vec<Polymer> = manual.rules.clone();
    let r2:Vec<Polymer> = r1.iter().map(|rule| rule.apply(&r1)).collect();
    let r4:Vec<Polymer> = r2.iter().map(|rule| rule.apply(&r2)).collect();
    let r8:Vec<Polymer> = r4.iter().map(|rule| rule.apply(&r4)).collect();


    let counts = manual.polymer.apply(&r8).apply(&r2).counts();
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

pub fn part_2(manual: &Manual) -> u64 {
    let r1: Vec<Polymer> = manual.rules.clone();
    let r2:Vec<Polymer> = r1.iter().map(|rule| rule.apply(&r1)).collect();
    let r4:Vec<Polymer> = r2.iter().map(|rule| rule.apply(&r2)).collect();
    let r8:Vec<Polymer> = r4.iter().map(|rule| rule.apply(&r4)).collect();
    let r16:Vec<Polymer> = r8.iter().map(|rule| rule.apply(&r8)).collect();
    let r32:Vec<Polymer> = r16.iter().map(|rule| rule.apply(&r16)).collect();



    let polymer = manual.polymer.apply(&r32).apply(&r8);



    let counts = polymer.counts();
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

    #[test]
    fn example_2_produces_2188189693529() {
        let manual = EXAMPLE.parse().unwrap();
        assert_eq!(2188189693529, part_2(&manual));
    }

    #[test]
    fn part_2_works() {
        let manual = parse_file("src/day14/input.txt");
        assert_eq!(3411, part_1(&manual));
    }
}
