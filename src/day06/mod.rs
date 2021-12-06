use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct School {
    fish: VecDeque<usize>,
}

impl FromStr for School {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut fish = VecDeque::from([0; 9]);
        for timer in input.split(',') {
            let timer: usize = timer.parse().unwrap();
            fish[timer] += 1;
        }
        Ok(Self { fish })
    }
}

impl School {
    fn tick(&mut self) {
        let spawn = self.fish.pop_front().unwrap();
        self.fish.push_back(spawn);
        self.fish[6] += spawn;
    }

    fn size(&self) -> usize {
        self.fish.iter().sum()
    }
}

pub fn part_1(mut school: School) -> usize {
    for _ in 0..80 {
        school.tick();
    }
    school.size()
}

pub fn part_2(mut school: School) -> usize {
    for _ in 0..256 {
        school.tick();
    }
    school.size()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_file;

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn example_1_produces_5934() {
        let input = EXAMPLE.parse().unwrap();
        assert_eq!(5934, part_1(input));
    }

    #[test]
    fn part_1_works() {
        let input = parse_file("src/day06/input.txt");
        assert_eq!(391_888, part_1(input));
    }

    #[test]
    fn example_2_produces_26984457539() {
        let input = EXAMPLE.parse().unwrap();
        assert_eq!(26_984_457_539, part_2(input));
    }

    #[test]
    fn part_2_works() {
        let input = parse_file("src/day06/input.txt");
        assert_eq!(1_754_597_645_339, part_2(input));
    }
}
