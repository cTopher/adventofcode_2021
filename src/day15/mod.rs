use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

type Position = (usize, usize);

#[derive(Clone, Debug, Copy, Hash)]
struct Node {
    risk_level: usize,
    g: usize,
    h: usize,
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

    fn get_mut(&mut self, (x, y): Position) -> &mut Node {
        &mut self.nodes[y][x]
    }

    pub fn solve(&mut self) -> usize {
        self.nodes[0][0].g = 0;
        let mut paths = BinaryHeap::new();
        paths.push(Path {
            position: (0, 0),
            g: 0,
            f: self.nodes[0][0].h,
        });
        while let Some(path) = paths.pop() {
            if path.position == (self.width - 1, self.height - 1) {
                return path.g;
            }
            for position in self.neighbours(path.position) {
                let node = self.get_mut(position);
                let g = path.g + node.risk_level;
                if node.g > g {
                    node.g = g;
                    paths.push(Path {
                        position,
                        g,
                        f: g + node.h,
                    });
                }
            }
        }
        panic!("No solution found");
    }
}

impl Cavern {
    pub fn new(input: CavernInput, size: usize) -> Self {
        let CavernInput(risk_levels) = input;
        let input_height = risk_levels.len();
        let input_width = risk_levels[0].len();
        let height = input_height * size;
        let width = input_width * size;
        let risk = |x: usize, y: usize| {
            let extra = x / input_width + y / input_height;
            (risk_levels[y % input_height][x % input_width] + extra -1) % 9 + 1
        };
        let h_start = risk(width - 1, height - 1) + height + width - 2;
        let nodes: Vec<Vec<Node>> = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| Node {
                        risk_level: risk(x, y),
                        g: usize::MAX,
                        h: h_start + x + y,
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

pub struct CavernInput(Vec<Vec<usize>>);

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
        Ok(Self(risk_levels))
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
