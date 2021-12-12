use std::collections::HashMap;
use std::str::FromStr;

#[derive(Default, Clone, Debug)]
pub struct CaveSystem {
    connections: HashMap<Cave, Vec<Cave>>,
}

impl CaveSystem {
    pub fn insert_connection(&mut self, cave1: Cave, cave2: Cave) {
        self.insert_single(cave1.clone(), cave2.clone());
        self.insert_single(cave2, cave1);
    }

    fn insert_single(&mut self, from: Cave, to: Cave) {
        if from != Cave::End && to != Cave::Start {
            self.connections
                .entry(from)
                .or_insert_with(Vec::new)
                .push(to);
        }
    }

    fn paths(&self, may_revisit: bool) -> Vec<Path> {
        self.find_paths(Path::new(may_revisit))
            .into_iter()
            .filter(Path::done)
            .collect()
    }

    fn find_paths(&self, path: Path) -> Vec<Path> {
        if path.done() {
            return vec![path];
        }
        self.connections
            .get(path.last())
            .unwrap()
            .iter()
            .filter_map(move |to| path.push(to))
            .flat_map(|path| self.find_paths(path))
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Cave {
    Start,
    End,
    Small(String),
    Large(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Path {
    small_caves: Vec<String>,
    may_revisit: bool,
    last: Cave,
}

impl Path {
    const fn new(may_revisit: bool) -> Self {
        Self {
            small_caves: Vec::new(),
            may_revisit,
            last: Cave::Start,
        }
    }

    const fn last(&self) -> &Cave {
        &self.last
    }

    fn contains(&self, small_cave: &String) -> bool {
        self.small_caves.contains(small_cave)
    }

    fn push(&self, cave: &Cave) -> Option<Self> {
        let revisit = if let Cave::Small(small_cave) = cave {
            self.contains(small_cave)
        } else {
            false
        };
        if !self.may_revisit && revisit {
            None
        } else {
            let mut new = Self {
                small_caves: self.small_caves.clone(),
                may_revisit: !revisit && self.may_revisit,
                last: cave.clone(),
            };
            if let Cave::Small(small_cave) = &cave {
                new.small_caves.push(small_cave.clone());
            };
            Some(new)
        }
    }

    fn done(&self) -> bool {
        self.last() == &Cave::End
    }
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "start" => Self::Start,
            "end" => Self::End,
            _ if input.to_lowercase() == input => Self::Small(input.to_owned()),
            _ => Self::Large(input.to_owned()),
        })
    }
}

impl FromStr for CaveSystem {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut system = Self::default();
        for line in input.lines() {
            let (a, b) = line.split_once('-').unwrap();
            system.insert_connection(a.parse().unwrap(), b.parse().unwrap());
        }
        Ok(system)
    }
}

pub fn part_1(system: &CaveSystem) -> usize {
    system.paths(false).len()
}

pub fn part_2(system: &CaveSystem) -> usize {
    system.paths(true).len()
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    use super::*;

    const MINI_EXAMPLE: &str = "\
        start-A\n\
        start-b\n\
        A-c\n\
        A-b\n\
        b-d\n\
        A-end\n\
        b-end\n\
    ";

    const LARGE_EXAMPLE: &str = "\
        fs-end\n\
        he-DX\n\
        fs-he\n\
        start-DX\n\
        pj-DX\n\
        end-zg\n\
        zg-sl\n\
        zg-pj\n\
        pj-he\n\
        RW-he\n\
        fs-DX\n\
        pj-RW\n\
        zg-RW\n\
        start-pj\n\
        he-WI\n\
        zg-he\n\
        pj-fs\n\
        start-RW\
    ";

    #[test]
    fn small_example_1_produces_10() {
        let system = MINI_EXAMPLE.parse().unwrap();
        assert_eq!(10, part_1(&system));
    }

    #[test]
    fn large_example_1_produces_10() {
        let system = LARGE_EXAMPLE.parse().unwrap();
        assert_eq!(226, part_1(&system));
    }

    #[test]
    fn part_1_works() {
        let system = parse_file("src/day12/input.txt");
        assert_eq!(5576, part_1(&system));
    }

    #[test]
    fn small_example_2_produces_36() {
        let system = MINI_EXAMPLE.parse().unwrap();
        assert_eq!(36, part_2(&system));
    }

    #[test]
    fn large_example_2_produces_3509() {
        let system = LARGE_EXAMPLE.parse().unwrap();
        assert_eq!(3509, part_2(&system));
    }

    #[test]
    fn part_2_works() {
        let system = parse_file("src/day12/input.txt");
        assert_eq!(152_837, part_2(&system));
    }
}