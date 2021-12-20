use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    algorithm: Vec<bool>,
    image: Image,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Image {
    pixels: Vec<Vec<bool>>,
    negative: bool,
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.pixels {
            for pixel in row {
                write!(f, "{}", if *pixel { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Image {
    #[allow(clippy::cast_possible_wrap)]
    fn enhance(&self, algorithm: &[bool]) -> Self {
        let negative = algorithm[0] ^ self.negative;
        let x_max = self.pixels[0].len() as isize;
        let y_max = self.pixels.len() as isize;
        let pixels = (-1..=y_max)
            .map(|y| {
                (-1..=x_max)
                    .map(|x| self.get_enhanced_pixel(x, y, algorithm))
                    .collect()
            })
            .collect();
        Self { pixels, negative }
    }

    fn get_enhanced_pixel(&self, x: isize, y: isize, algorithm: &[bool]) -> bool {
        let binary = self.get_binary_number(x, y);
        algorithm[binary]
    }

    fn get_binary_number(&self, tx: isize, ty: isize) -> usize {
        let mut result = 0;
        for y in ty - 1..=ty + 1 {
            for x in tx - 1..=tx + 1 {
                result <<= 1;
                if self.is_lit(x, y) {
                    result |= 1;
                }
            }
        }
        result
    }

    #[allow(clippy::cast_sign_loss)]
    fn is_lit(&self, tx: isize, ty: isize) -> bool {
        if tx < 0 || ty < 0 {
            self.negative
        } else {
            self.pixels
                .get(ty as usize)
                .and_then(|row| row.get(tx as usize))
                .copied()
                .unwrap_or(self.negative)
        }
    }

    fn count(&self) -> usize {
        self.pixels.iter().flatten().filter(|&&x| x).count()
    }
}

impl FromStr for Image {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let pixels = input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        Ok(Self {
            pixels,
            negative: false,
        })
    }
}

impl FromStr for Input {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (algorithm, image) = input.split_once("\n\n").unwrap();
        let algorithm = algorithm.chars().map(|c| c == '#').collect();
        Ok(Self {
            algorithm,
            image: image.parse().unwrap(),
        })
    }
}

pub fn part_1(input: Input) -> usize {
    enhance(input, 2).count()
}

pub fn part_2(input: Input) -> usize {
    enhance(input, 50).count()
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
