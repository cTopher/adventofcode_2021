use std::collections::HashSet;
use std::str::FromStr;

#[derive(Default, Clone, Debug)]
pub struct HeightMap {
    heights: Vec<Vec<u32>>,
}

#[derive(Clone)]
pub struct Entry<'a> {
    position: Position,
    height: u32,
    map: &'a HeightMap,
}

#[derive(Default, Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl<'a> Entry<'a> {
    pub const fn x(&self) -> usize {
        self.position.x
    }

    pub const fn y(&self) -> usize {
        self.position.y
    }

    pub fn up(&self) -> Option<Self> {
        if self.y() == 0 {
            None
        } else {
            self.map.entry(Position::new(self.x(), self.y() - 1))
        }
    }

    pub fn down(&self) -> Option<Self> {
        self.map.entry(Position::new(self.x(), self.y() + 1))
    }

    pub fn left(&self) -> Option<Self> {
        if self.x() == 0 {
            None
        } else {
            self.map.entry(Position::new(self.x() - 1, self.y()))
        }
    }

    pub fn right(&self) -> Option<Self> {
        self.map.entry(Position::new(self.x() + 1, self.y()))
    }

    pub fn neighbours(&self) -> Vec<Self> {
        [self.up(), self.down(), self.left(), self.right()]
            .into_iter()
            .flatten()
            .collect()
    }

    pub fn is_low_point(&self) -> bool {
        let neighbours = self.neighbours();
        neighbours.into_iter().all(|n| self.height < n.height)
    }

    pub fn basin(&self) -> HashSet<Position> {
        let mut basin: HashSet<Position> = self
            .neighbours()
            .into_iter()
            .filter(|n| n.flow_to() == Some(self.position))
            .flat_map(|n| n.basin())
            .collect();
        basin.insert(self.position);
        basin
    }

    pub fn flow_to(&self) -> Option<Position> {
        if self.height == 9 {
            None
        } else {
            self.neighbours()
                .into_iter()
                .filter(|n| n.height < self.height)
                .min_by_key(|n| n.height)
                .map(|n| n.position)
        }
    }
}

impl HeightMap {
    pub fn entries(&self) -> impl Iterator<Item = Entry> + '_ {
        self.heights.iter().enumerate().flat_map(move |(y, row)| {
            row.iter().enumerate().map(move |(x, &height)| Entry {
                position: Position { x, y },
                height,
                map: self,
            })
        })
    }

    pub fn entry(&self, position: Position) -> Option<Entry> {
        self.heights.get(position.y).and_then(|row| {
            row.get(position.x).map(|&height| Entry {
                position,
                height,
                map: self,
            })
        })
    }

    pub fn low_points(&self) -> impl Iterator<Item = Entry<'_>> + '_ {
        self.entries().filter(Entry::is_low_point)
    }
}

impl FromStr for HeightMap {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let heights: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        Ok(Self { heights })
    }
}

pub fn part_1(map: &HeightMap) -> u32 {
    map.low_points().map(|entry| entry.height + 1).sum()
}

pub fn part_2(map: &HeightMap) -> usize {
    let mut basin_sizes: Vec<usize> = map.low_points().map(|entry| entry.basin().len()).collect();
    basin_sizes.sort_unstable();
    basin_sizes.into_iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    use super::*;

    const EXAMPLE: &str = "\
        2199943210\n\
        3987894921\n\
        9856789892\n\
        8767896789\n\
        9899965678\
    ";

    #[test]
    fn example_1_produces_26() {
        let map = EXAMPLE.parse().unwrap();
        assert_eq!(15, part_1(&map));
    }

    #[test]
    fn part_1_works() {
        let map = parse_file("src/day09/input.txt");
        assert_eq!(600, part_1(&map));
    }

    #[test]
    fn example_2_produces_1134() {
        let map = EXAMPLE.parse().unwrap();
        assert_eq!(1134, part_2(&map));
    }

    #[test]
    fn part_2_works() {
        let map = parse_file("src/day09/input.txt");
        assert_eq!(987_840, part_2(&map));
    }
}
