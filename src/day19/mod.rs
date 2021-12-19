use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Summary {
    scanners: Vec<Scanner>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scanner {
    beacons: Vec<Vector>,
    connections: Vec<Connection>,
    distances: HashSet<i32>,
}

impl Scanner {
    fn overlap(&self, other: &Scanner) -> usize {
        self.distances.intersection(&other.distances).count()
    }
}

impl From<Vec<Vector>> for Scanner {
    fn from(beacons: Vec<Vector>) -> Self {
        let connections: Vec<Connection> = (0..beacons.len())
            .flat_map(|i| (i + 1..beacons.len()).map(move |j| (i, j)))
            .map(|(i, j)| Connection::new(beacons[i], beacons[j]))
            .collect();
        let distances = connections.iter().map(|c| c.distance).collect();
        Self {
            beacons,
            connections,
            distances,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct Connection {
    a: Vector,
    b: Vector,
    distance: i32,
}

impl Connection {
    fn new(a: Vector, b: Vector) -> Self {
        let distance = a.distance(b);
        Connection { a, b, distance }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    fn distance(&self, other: Vector) -> i32 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

impl FromStr for Vector {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut numbers = input.splitn(3, ',').map(|s| s.parse::<i32>().unwrap());
        let x = numbers.next().unwrap();
        let y = numbers.next().unwrap();
        let z = numbers.next().unwrap();
        assert_eq!(numbers.next(), None);
        Ok(Self { x, y, z })
    }
}

impl FromStr for Scanner {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let beacons: Vec<Vector> = input
            .lines()
            .skip(1)
            .map(|line| line.parse().unwrap())
            .collect();
        Ok(beacons.into())
    }
}

impl FromStr for Summary {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let scanners = input.split("\n\n").map(|s| s.parse().unwrap()).collect();
        Ok(Self { scanners })
    }
}

pub fn part_1(summary: Summary) -> usize {
    let scanner0 = &summary.scanners[0];

    for i in 0..summary.scanners.len() {
        println!("{} {}", i, summary.scanners[i].overlap(scanner0));
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::{parse_file, parse_file_lines, parse_str_lines};

    use super::*;

    #[test]
    fn example_1_produces_79() {
        let summary = parse_file("src/day19/example.txt");
        assert_eq!(79, part_1(summary));
    }

    #[test]
    fn part_1_works() {
        let summary = parse_file("src/day19/input.txt");
        assert_eq!(79, part_1(summary));
    }
}
