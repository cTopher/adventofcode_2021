use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use vector::Vector;

mod vector;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BeaconMap {
    unmapped_scanners: Vec<Scanner>,
    scanner_positions: Vec<Vector>,
}

impl BeaconMap {
    fn solve(&mut self) -> Scanner {
        let mut scanners: VecDeque<Scanner> = self.unmapped_scanners.drain(..).collect();
        let mut scanner_0 = scanners.pop_front().unwrap();
        while let Some(scanner) = scanners.pop_front() {
            if scanner.overlap(&scanner_0) >= 66 {
                let pos = scanner_0.merge(scanner);
                self.scanner_positions.push(pos);
            } else {
                scanners.push_back(scanner);
            }
        }
        scanner_0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scanner {
    beacons: Vec<Vector>,
    distances_per_beacon: Vec<HashSet<i32>>,
    all_distances: HashSet<i32>,
}

impl Scanner {
    fn overlap(&self, other: &Self) -> usize {
        self.all_distances
            .intersection(&other.all_distances)
            .count()
    }

    fn merge(&mut self, mut other: Self) -> Vector {
        let (orientation, translation) = self.get_transformation(&other);
        other.apply_transformation(orientation, translation);
        for beacon in other.beacons {
            if !self.beacons.contains(&beacon) {
                self.beacons.push(beacon);
            }
        }
        self.update();
        translation
    }

    fn apply_transformation(&mut self, orientation: u8, translation: Vector) {
        for beacon in &mut self.beacons {
            *beacon = beacon.orientation(orientation) + translation;
        }
    }

    fn get_transformation(&mut self, other: &Self) -> (u8, Vector) {
        let common: Vec<(Vector, Vector)> = (0..self.beacons.len())
            .flat_map(|i| (0..other.beacons.len()).map(move |j| (i, j)))
            .filter(|&(i, j)| {
                self.distances_per_beacon[i]
                    .intersection(&other.distances_per_beacon[j])
                    .count()
                    >= 11
            })
            .map(|(i, j)| (self.beacons[i], other.beacons[j]))
            .take(2)
            .collect();
        for orientation in 0..24 {
            let delta0 = common[0].1.orientation(orientation) - common[0].0;
            let delta1 = common[1].1.orientation(orientation) - common[1].0;
            if delta0 == delta1 {
                return (orientation, -delta0);
            }
        }
        panic!("no orientation found");
    }

    fn update(&mut self) {
        self.distances_per_beacon = self
            .beacons
            .iter()
            .map(|&a| {
                self.beacons
                    .iter()
                    .map(|&b| a.square_distance(b))
                    .filter(|&d| d != 0)
                    .collect()
            })
            .collect();
        self.all_distances = self
            .distances_per_beacon
            .iter()
            .flatten()
            .copied()
            .collect();
    }
}

impl From<Vec<Vector>> for Scanner {
    fn from(beacons: Vec<Vector>) -> Self {
        let mut result = Self {
            beacons,
            distances_per_beacon: Vec::new(),
            all_distances: HashSet::new(),
        };
        result.update();
        result
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

impl FromStr for BeaconMap {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let scanners = input.split("\n\n").map(|s| s.parse().unwrap()).collect();
        Ok(Self {
            unmapped_scanners: scanners,
            scanner_positions: Vec::new(),
        })
    }
}

pub fn part_1(mut map: BeaconMap) -> usize {
    let scanner = map.solve();
    scanner.beacons.len()
}

pub fn part_2(mut map: BeaconMap) -> i32 {
    map.solve();
    map.scanner_positions
        .iter()
        .flat_map(|a| {
            map.scanner_positions
                .iter()
                .map(|&b| a.manhattan_distance(b))
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    use super::*;

    #[test]
    fn example_1_produces_79() {
        let summary = parse_file("src/day19/example.txt");
        assert_eq!(79, part_1(summary));
    }

    #[test]
    fn part_1_works() {
        let summary = parse_file("src/day19/input.txt");
        assert_eq!(323, part_1(summary));
    }

    #[test]
    fn example_2_produces_3621() {
        let summary = parse_file("src/day19/example.txt");
        assert_eq!(3621, part_2(summary));
    }

    #[test]
    fn part_2_works() {
        let summary = parse_file("src/day19/input.txt");
        assert_eq!(10685, part_2(summary));
    }
}
