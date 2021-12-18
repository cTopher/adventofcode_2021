use std::fmt;
use std::fmt::Formatter;
use std::ops::Add;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Snailfish {
    Number(u32),
    Pair(Box<Self>, Box<Self>),
}

impl Add for Snailfish {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::Pair(Box::new(self), Box::new(rhs));
        result.reduce();
        result
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
enum Explosion {
    Fizzle,
    Boom(Option<u32>, Option<u32>),
}

impl Snailfish {
    fn reduce(&mut self) {
        loop {
            while self.explode(0) != Explosion::Fizzle {}
            if !self.split() {
                return;
            };
        }
    }

    fn explode(&mut self, level: usize) -> Explosion {
        match self {
            Snailfish::Number(_) => Explosion::Fizzle,
            Snailfish::Pair(a, b) if level == 4 => {
                let a = a.unwrap_number();
                let b = b.unwrap_number();
                *self = Self::default();
                Explosion::Boom(Some(a), Some(b))
            }
            Snailfish::Pair(left, right) => {
                if let Explosion::Boom(a, b) = left.explode(level + 1) {
                    if let Some(b) = b {
                        right.add_left(b);
                    }
                    Explosion::Boom(a, None)
                } else if let Explosion::Boom(a, b) = right.explode(level + 1) {
                    if let Some(a) = a {
                        left.add_right(a);
                    }
                    Explosion::Boom(None, b)
                } else {
                    Explosion::Fizzle
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Snailfish::Number(x) if *x >= 10 => {
                let left = *x / 2;
                let right = left + *x % 2;
                *self = Self::Pair(Box::new(Self::Number(left)), Box::new(Self::Number(right)));
                true
            }
            Self::Number(_) => false,
            Snailfish::Pair(left, right) => left.split() || right.split(),
        }
    }

    fn add_right(&mut self, x: u32) {
        match self {
            Snailfish::Number(nb) => *nb += x,
            Snailfish::Pair(_, right) => {
                right.add_right(x);
            }
        }
    }

    fn add_left(&mut self, x: u32) {
        match self {
            Snailfish::Number(nb) => *nb += x,
            Snailfish::Pair(left, _) => {
                left.add_left(x);
            }
        }
    }

    fn unwrap_number(&self) -> u32 {
        match self {
            Snailfish::Number(number) => *number,
            Snailfish::Pair(_, _) => panic!("unwrap_number called on {:?}", self),
        }
    }

    fn from_iterator(chars: &mut impl Iterator<Item = char>) -> Self {
        let next = chars.next().unwrap();
        if next == '[' {
            let first = Self::from_iterator(chars);
            assert_eq!(chars.next(), Some(','));
            let second = Self::from_iterator(chars);
            assert_eq!(chars.next(), Some(']'));
            Self::Pair(Box::new(first), Box::new(second))
        } else {
            Self::Number(next.to_digit(10).unwrap())
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Snailfish::Number(nb) => *nb,
            Snailfish::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl From<u32> for Snailfish {
    fn from(x: u32) -> Self {
        Self::Number(x)
    }
}

impl Default for Snailfish {
    fn default() -> Self {
        0.into()
    }
}

impl fmt::Display for Snailfish {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Snailfish::Number(x) => write!(f, "{}", x),
            Snailfish::Pair(a, b) => write!(f, "[{},{}]", a, b),
        }
    }
}

impl FromStr for Snailfish {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let chars = &mut input.chars();
        let result = Self::from_iterator(chars);
        assert_eq!(chars.next(), None);
        Ok(result)
    }
}

pub fn part_1(snailfish: impl Iterator<Item = Snailfish>) -> u32 {
    snailfish.reduce(|a, b| a + b).unwrap().magnitude()
}

pub fn part_2(snailfish: impl Iterator<Item = Snailfish>) -> u32 {
    let snailfish: Vec<_> = snailfish.collect();
    let len = snailfish.len();
    (0..len)
        .flat_map(|a| (0..len).map(move |b| (a, b)))
        .filter(|(a, b)| a != b)
        //TODO optimize this shit
        .map(|(a, b)| ((&snailfish)[a].clone() + (&snailfish)[b].clone()).magnitude())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{parse_file_lines, parse_str_lines};

    use super::*;

    const EXAMPLE: &str = "\
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n\
        [[[5,[2,8]],4],[5,[[9,9],0]]]\n\
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n\
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n\
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n\
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n\
        [[[[5,4],[7,7]],8],[[8,3],8]]\n\
        [[9,3],[[9,9],[6,[4,9]]]]\n\
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n\
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]\n\
    ";

    #[test]
    fn example_1_produces_4140() {
        let snailfish = parse_str_lines(EXAMPLE);
        assert_eq!(4140, part_1(snailfish));
    }

    #[test]
    fn part_1_works() {
        let snailfish = parse_file_lines("src/day18/input.txt");
        assert_eq!(3305, part_1(snailfish));
    }

    #[test]
    fn example_2_produces_3993() {
        let snailfish = parse_str_lines(EXAMPLE);
        assert_eq!(3993, part_2(snailfish));
    }

    #[test]
    fn part_2_works() {
        let snailfish = parse_file_lines("src/day18/input.txt");
        assert_eq!(4563, part_2(snailfish));
    }
}
