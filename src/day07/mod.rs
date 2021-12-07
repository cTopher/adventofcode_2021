use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Crabs {
    heights: Vec<u32>,
}

impl Crabs {
    fn median(&mut self) -> u32 {
        self.heights.sort_unstable();
        let len = self.heights.len();
        if len % 2 == 0 {
            (self.heights[len / 2 - 1] + self.heights[len / 2]) / 2
        } else {
            self.heights[len / 2]
        }
    }

    fn min(&self) -> u32 {
        *self.heights.iter().min().unwrap()
    }

    fn max(&self) -> u32 {
        *self.heights.iter().max().unwrap()
    }

    fn total_align_distance(&self, align_height: u32) -> u32 {
        self.heights
            .iter()
            .map(|&height| delta(height, align_height))
            .sum()
    }

    fn align_cost(&self, align_height: u32) -> u32 {
        self.heights
            .iter()
            .map(|&height| cost(height, align_height))
            .sum()
    }
}

const fn cost(a: u32, b: u32) -> u32 {
    let delta = delta(a, b);
    (1 + delta) * delta / 2
}

const fn delta(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

impl FromStr for Crabs {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let heights = input.split(',').map(|s| s.parse().unwrap()).collect();
        Ok(Self { heights })
    }
}

pub fn part_1(mut crabs: Crabs) -> u32 {
    let median = crabs.median();
    println!("median: {}", median);
    crabs.total_align_distance(median)
}

/// really stupid implementation because I was lazy
/// a proper solution could be to use regression maybe?
pub fn part_2(crabs: &Crabs) -> u32 {
    let guess = (crabs.max() - crabs.min()) / 2;
    let mut cost = u32::MAX;
    for height in (0..guess).rev() {
        let new_cost = crabs.align_cost(height);
        println!("{} -> {}", height, new_cost);
        if new_cost <= cost {
            cost = new_cost;
        } else {
            break;
        }
    }
    cost
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_file;

    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn example_1_produces_37() {
        let crabs: Crabs = EXAMPLE.parse().unwrap();
        assert_eq!(37, part_1(crabs));
    }

    #[test]
    fn part_1_works() {
        let crabs: Crabs = parse_file("src/day07/input.txt");
        assert_eq!(352_254, part_1(crabs));
    }

    #[test]
    fn example_2_produces_168() {
        let crabs: Crabs = EXAMPLE.parse().unwrap();
        assert_eq!(168, part_2(&crabs));
    }

    #[test]
    fn part_2_works() {
        let crabs: Crabs = parse_file("src/day07/input.txt");
        assert_eq!(99_053_143, part_2(&crabs));
    }
}
