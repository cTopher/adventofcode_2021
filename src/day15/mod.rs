use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

type Position = (usize, usize);

#[derive(Clone, Debug, Copy, Hash)]
struct Node {
    risk_level: usize,
    total_risk: usize,
}

#[derive(Clone, Debug)]
pub struct Cavern {
    nodes: Vec<Vec<Node>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug, Copy, Hash, Eq, PartialEq)]
struct Path {
    position: Position,
    total_risk: usize,
}

impl Ord for Path {
    /// Path B > A when `B.total_risk` < `A.total_risk` because a lower risk is better
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_risk.cmp(&self.total_risk)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Cavern {
    fn neighbours(&self, (x, y): Position) -> impl Iterator<Item = Position> + 'static {
        let up = if y > 0 { Some((x, y - 1)) } else { None };
        let right = if x < self.width - 1 {
            Some((x + 1, y))
        } else {
            None
        };
        let down = if y < self.height - 1 {
            Some((x, y + 1))
        } else {
            None
        };
        let left = if x > 0 { Some((x - 1, y)) } else { None };
        [up, right, down, left].into_iter().flatten()
    }

    fn is_goal(&self, position: Position) -> bool {
        position == (self.width - 1, self.height - 1)
    }

    pub fn solve(&mut self) -> usize {
        self.nodes[0][0].total_risk = 0;
        let mut paths = BinaryHeap::new();
        paths.push(Path {
            position: (0, 0),
            total_risk: 0,
        });
        while let Some(path) = paths.pop() {
            if self.is_goal(path.position) {
                return path.total_risk;
            }
            for position @ (x, y) in self.neighbours(path.position) {
                let node = &mut self.nodes[y][x];
                let total_risk = path.total_risk + node.risk_level;
                if total_risk < node.total_risk {
                    node.total_risk = total_risk;
                    paths.push(Path {
                        position,
                        total_risk,
                    });
                }
            }
        }
        panic!("No solution found");
    }
}

impl Cavern {
    pub fn new(input: CavernInput, size: usize) -> Self {
        let risk_levels = input.risk_levels;
        let input_height = risk_levels.len();
        let input_width = risk_levels[0].len();
        let height = input_height * size;
        let width = input_width * size;
        let risk = |x: usize, y: usize| {
            let extra = x / input_width + y / input_height;
            (risk_levels[y % input_height][x % input_width] + extra - 1) % 9 + 1
        };
        let nodes: Vec<Vec<Node>> = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| Node {
                        risk_level: risk(x, y),
                        total_risk: usize::MAX,
                    })
                    .collect()
            })
            .collect();
        Self {
            nodes,
            width,
            height,
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct CavernInput {
    risk_levels: Vec<Vec<usize>>,
}

impl FromStr for CavernInput {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let risk_levels = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        Ok(Self { risk_levels })
    }
}

pub fn part_1(input: CavernInput) -> usize {
    Cavern::new(input, 1).solve()
}

pub fn part_2(input: CavernInput) -> usize {
    Cavern::new(input, 5).solve()
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    use super::*;

    const EXAMPLE: &str = "\
        1163751742\n\
        1381373672\n\
        2136511328\n\
        3694931569\n\
        7463417111\n\
        1319128137\n\
        1359912421\n\
        3125421639\n\
        1293138521\n\
        2311944581\n\
    ";

    #[test]
    fn example_1_produces_40() {
        let input = EXAMPLE.parse().unwrap();
        assert_eq!(40, part_1(input));
    }

    #[test]
    fn part_1_works() {
        let input = parse_file("src/day15/input.txt");
        assert_eq!(498, part_1(input));
    }

    #[test]
    fn example_2_produces_315() {
        let input = EXAMPLE.parse().unwrap();
        assert_eq!(315, part_2(input));
    }

    #[test]
    fn part_2_works() {
        let input = parse_file("src/day15/input.txt");
        assert_eq!(2901, part_2(input));
    }
}
