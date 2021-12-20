use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

pub struct Input {
    algorithm: Vec<bool>,
    image: Image,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Image {
    pixels: HashSet<Position>,
    negative:bool,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Image {
    fn new(negative:bool) -> Self{
        Self {
            pixels: HashSet::new(),
            negative,
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
        }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in self.y_min..=self.y_max {
            for x in self.x_min..=self.x_max {
                if self.negative ^ self.pixels.contains(&Position { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Image {
    fn push(&mut self, position: Position) {
        self.x_min = self.x_min.min(position.x);
        self.x_max = self.x_max.max(position.x);
        self.y_min = self.y_min.min(position.y);
        self.y_max = self.y_max.max(position.y);
        self.pixels.insert(position);
    }

    fn enhance(&self, algorithm: &[bool]) -> Self {
        let negative = algorithm[0] ^ self.negative;
        let mut result = Self::new(negative);
        for y in self.y_min - 1..=self.y_max + 1 {
            for x in self.x_min - 1..=self.x_max + 1 {
                let position = Position { x, y };
                let binary = self.get_binary_number(position);
                if negative ^ algorithm[binary] {
                    result.push(position);
                }
            }
        }
        result
    }

    fn get_binary_number(&self, position: Position) -> usize {
        let mut result = 0;
        for y in position.y - 1..=position.y + 1 {
            for x in position.x - 1..=position.x + 1 {
                result <<= 1;
                if self.negative ^ self.pixels.contains(&Position { x, y }) {
                    result |= 1;
                }
            }
        }
        result
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct Position {
    x: isize,
    y: isize,
}

impl FromStr for Image {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut image = Self::new(false);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    image.push(Position {
                        x: x as isize,
                        y: y as isize,
                    });
                }
            }
        }
        Ok(image)
    }
}

impl FromStr for Input {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (algorithm, image) = input.split_once("\n\n").unwrap();

        let algorithm = algorithm
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => panic!("Invalid character in input"),
            })
            .collect();
        Ok(Self {
            algorithm,
            image: image.parse().unwrap(),
        })
    }
}

pub fn part_1(input: Input) -> usize {
    enhance(input, 2).pixels.len()
}

pub fn part_2(input: Input) -> usize {
    enhance(input, 50).pixels.len()
}

fn enhance(input: Input, times: usize) -> Image {
    let mut image = input.image;
    for _ in 0..times {
        image = image.enhance(&input.algorithm);
    }
    image
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    use super::*;

    const EXAMPLE: &str = "\
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\
        \n\
        #..#.\n\
        #....\n\
        ##..#\n\
        ..#..\n\
        ..###\
    ";

    #[test]
    fn example_1_produces_35() {
        let input = EXAMPLE.parse().unwrap();
        assert_eq!(35, part_1(input));
    }

    #[test]
    fn part_1_works() {
        let input = parse_file("src/day20/input.txt");
        assert_eq!(5057, part_1(input));
    }

    #[test]
    fn example_2_produces_3351() {
        let input = EXAMPLE.parse().unwrap();
        assert_eq!(3351, part_2(input));
    }

    #[test]
    fn part_2_works() {
        let input = parse_file("src/day20/input.txt");
        assert_eq!(18502, part_2(input));
    }
}
