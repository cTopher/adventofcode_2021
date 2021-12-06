use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct School {
    fish: [u64; 9],
    // index of the fish in the array that have timer=0
    zero_index: usize,
}

impl FromStr for School {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut fish = [0; 9];
        for timer in input.split(',') {
            let timer: usize = timer.parse().unwrap();
            fish[timer] += 1;
        }
        Ok(Self {
            fish,
            zero_index: 0,
        })
    }
}

impl School {
    fn tick(&mut self) {
        let new = self.fish[self.zero_index];
        self.zero_index = (self.zero_index + 1) % 9;
        self.fish[(self.zero_index + 6) % 9] += new;
    }

    fn size(&self) -> u64 {
        self.fish.iter().sum()
    }
}

pub fn part_1(mut school: School) -> u64 {
    for _ in 0..80 {
        school.tick();
    }
    school.size()
}

pub fn part_2(mut school: School) -> u64 {
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
