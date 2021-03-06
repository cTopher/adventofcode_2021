use std::collections::HashMap;
use std::str::FromStr;

type Chain = (char, char);

#[derive(Clone, Debug)]
struct Polymer {
    chains: HashMap<Chain, u64>,
    last_element: char,
}

impl Polymer {
    fn apply_n(&mut self, rules: &[PairInsertion], n: usize) {
        for _ in 0..n {
            self.apply(rules);
        }
    }

    fn apply(&mut self, rules: &[PairInsertion]) {
        let mut new_chains: HashMap<Chain, u64> = HashMap::new();
        for rule in rules {
            if let Some(count) = self.chains.remove(&rule.from) {
                *new_chains.entry(rule.to.0).or_default() += count;
                *new_chains.entry(rule.to.1).or_default() += count;
            }
        }
        for (chain, count) in new_chains {
            *self.chains.entry(chain).or_default() += count;
        }
    }

    pub fn elements(&self) -> HashMap<char, u64> {
        let mut elements: HashMap<char, u64> = HashMap::from([(self.last_element, 1)]);
        for (&(a, _), &count) in &self.chains {
            *elements.entry(a).or_insert(0) += count;
        }
        elements
    }
}

#[derive(Clone, Debug)]
pub struct Manual {
    polymer_template: Polymer,
    rules: Vec<PairInsertion>,
}

#[derive(Clone, Debug, Copy)]
struct PairInsertion {
    from: Chain,
    to: (Chain, Chain),
}

impl FromStr for Polymer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let mut chains: HashMap<Chain, u64> = HashMap::new();
        for window in chars.windows(2) {
            *chains.entry((window[0], window[1])).or_insert(0) += 1;
        }
        let last_char = *chars.last().unwrap();
        Ok(Self {
            chains,
            last_element: last_char,
        })
    }
}

impl FromStr for Manual {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (polymer, rules) = input.split_once("\n\n").unwrap();
        Ok(Self {
            polymer_template: polymer.parse().unwrap(),
            rules: rules.lines().map(|line| line.parse().unwrap()).collect(),
        })
    }
}

impl FromStr for PairInsertion {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (pair, insertion) = input.split_once(" -> ").unwrap();
        let mut pair = pair.chars();
        let a = pair.next().unwrap();
        let b = pair.next().unwrap();
        let c = insertion.chars().next().unwrap();
        Ok(Self {
            from: (a, b),
            to: ((a, c), (c, b)),
        })
    }
}

pub fn part_1(manual: &Manual) -> u64 {
    let mut polymer = manual.polymer_template.clone();
    polymer.apply_n(&manual.rules, 10);
    let counts = polymer.elements();
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

pub fn part_2(manual: &Manual) -> u64 {
    let mut polymer = manual.polymer_template.clone();
    polymer.apply_n(&manual.rules, 40);
    let counts = polymer.elements();
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
        assert_eq!(2_188_189_693_529, part_2(&manual));
    }

    #[test]
    fn part_2_works() {
        let manual = parse_file("src/day14/input.txt");
        assert_eq!(7_477_815_755_570, part_2(&manual));
    }
}
