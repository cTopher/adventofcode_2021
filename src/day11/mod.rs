use std::cmp::min;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Default, Clone, Debug)]
pub struct Cavern {
    octopuses: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

type Position = (usize, usize);

impl Cavern {
    fn new(octopuses: Vec<Vec<u8>>) -> Self {
        let width = octopuses[0].len();
        let height = octopuses.len();
        Self {
            octopuses,
            width,
            height,
        }
    }

    pub fn neighbours(&self, (cx, cy): Position) -> impl Iterator<Item = Position> + '_ {
        let x_min = cx.saturating_sub(1);
        let x_max = min(cx + 1, self.width - 1);
        let y_min = cy.saturating_sub(1);
        let y_max = min(cy + 1, self.height - 1);
        (x_min..=x_max).flat_map(move |x| {
            (y_min..=y_max)
                .filter(move |&y| x != cx || y != cy)
                .map(move |y| (x, y))
        })
    }

    fn positions(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| (x, y)))
    }

    fn tick(&mut self) -> usize {
        let mut to_tick: Vec<Position> = self.positions().collect();
        let mut flashed: HashSet<Position> = HashSet::new();
        while let Some(position @ (x, y)) = to_tick.pop() {
            self.octopuses[x][y] += 1;
            if self.octopuses[x][y] == 10 {
                to_tick.extend(self.neighbours(position));
                flashed.insert(position);
            }
        }
        let number_of_flashes = flashed.len();
        for (x, y) in flashed {
            self.octopuses[x][y] = 0;
        }
        number_of_flashes
    }
}

impl FromStr for Cavern {
    type Err = ();

    #[allow(clippy::cast_possible_truncation)]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let octopuses = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();
        Ok(Self::new(octopuses))
    }
}

pub fn part_1(mut map: Cavern) -> usize {
    (0..100).map(|_| map.tick()).sum()
}

#[allow(clippy::maybe_infinite_iter)]
pub fn part_2(mut map: Cavern) -> usize {
    (1..)
        .find(|_| map.tick() == map.width * map.height)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    use super::*;

    const EXAMPLE: &str = "\
        5483143223\n\
        2745854711\n\
        5264556173\n\
        6141336146\n\
        6357385478\n\
        4167524645\n\
        2176841721\n\
        6882881134\n\
        4846848554\n\
        5283751526\
    ";

    #[test]
    fn example_1_produces_1656() {
        let map = EXAMPLE.parse().unwrap();
        assert_eq!(1656, part_1(map));
    }

    #[test]
    fn part_1_works() {
        let map = parse_file("src/day11/input.txt");
        assert_eq!(1688, part_1(map));
    }

    #[test]
    fn example_2_produces_195() {
        let map = EXAMPLE.parse().unwrap();
        assert_eq!(195, part_2(map));
    }

    #[test]
    fn part_2_works() {
        let map = parse_file("src/day11/input.txt");
        assert_eq!(403, part_2(map));
    }
}
