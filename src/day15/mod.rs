use std::cmp::{ Ordering};
use std::collections::{BinaryHeap};
use std::str::FromStr;

type Position = (usize, usize);

#[derive(Clone, Debug, Copy, Hash)]
struct Node {
    risk_level: usize,
    g: usize,
    h: usize,
    f: usize,
}

impl Node {
    fn set_g(&mut self, g: usize) {
        self.g = g;
        self.f = g + self.h;
    }
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
    g: usize,
    f: usize,
}

/// Path A < B when B.f < A.f
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Cavern {
    //TODO iterator
    fn neighbours(&self, (x, y): Position) -> Vec<Position> {
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
        [up, right, down, left].into_iter().flatten().collect()
    }

    fn get_mut(&mut self, (x, y): Position) -> &mut Node {
        &mut self.nodes[y][x]
    }

    pub fn solve(&mut self) -> usize {
        let mut paths = BinaryHeap::new();
        paths.push(Path {
            position: (0, 0),
            g: 0,
            f: self.nodes[0][0].f,
        });
        while let Some(path) = paths.pop() {
            if path.position == (self.width - 1, self.height - 1) {
                return path.g;
            }
            for position in self.neighbours(path.position) {
                let node = self.get_mut(position);
                let g = path.g + node.risk_level;
                if node.g > g {
                    node.set_g(g);
                    paths.push(Path {
                        position,
                        g,
                        f: node.f,
                    });
                }
            }
        }
        panic!("No solution found");
    }
}

impl From<Vec<Vec<usize>>> for Cavern {
    fn from(risk_levels: Vec<Vec<usize>>) -> Self {
        let height = risk_levels.len();
        let width = risk_levels[0].len();
        let h_start = risk_levels[height - 1][width - 1] + height + width - 2;
        let mut nodes: Vec<Vec<Node>> = risk_levels
            .into_iter()
            .enumerate()
            .map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(x, risk_level)| Node {
                        risk_level,
                        g: usize::MAX,
                        h: h_start + x + y,
                        f: usize::MAX,
                    })
                    .collect()
            })
            .collect();
        nodes[0][0].set_g(0);
        Cavern {
            nodes,
            width,
            height,
        }
    }
}

impl FromStr for Cavern {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let risk_levels: Vec<Vec<usize>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        Ok(risk_levels.into())
    }
}

pub fn part_1(mut cavern: Cavern) -> usize {
    cavern.solve()
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
        let cavern = EXAMPLE.parse().unwrap();
        assert_eq!(40, part_1(cavern));
    }

    #[test]
    fn part_1_works() {
        let cavern = parse_file("src/day15/input.txt");
        assert_eq!(0, part_1(cavern));
    }

}
