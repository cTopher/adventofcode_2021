use board::Board;
use std::str::FromStr;

mod board;

#[derive(Debug)]
pub struct Game {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

impl Game {
    pub fn play_to_win(&mut self) -> u32 {
        for &number in &self.numbers {
            for board in &mut self.boards {
                board.apply(number);
                if board.victory() {
                    return board.score(u32::from(number));
                }
            }
        }
        panic!("No victory");
    }

    pub fn play_to_lose(&mut self) -> u32 {
        let mut open_boards = self.boards.len();
        for &number in &self.numbers {
            for board in self.boards.iter_mut().filter(|board| !board.victory()) {
                board.apply(number);
                if board.victory() {
                    if open_boards == 1 {
                        return board.score(u32::from(number));
                    }
                    open_boards -= 1;
                }
            }
        }
        panic!("No victory");
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut split = input.split("\n\n");
        Ok(Self {
            numbers: split
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect(),
            boards: split.map(|s| s.parse().unwrap()).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_file;

    const EXAMPLE: &str = "\
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
        \n\
        22 13 17 11  0\n\
         8  2 23  4 24\n\
        21  9 14 16  7\n\
         6 10  3 18  5\n\
         1 12 20 15 19\n\
        \n\
         3 15  0  2 22\n\
         9 18 13 17  5\n\
        19  8  7 25 23\n\
        20 11 10 24  4\n\
        14 21 16 12  6\n\
        \n\
        14 21 17 24  4\n\
        10 16 15  9 19\n\
        18  8 23 26 20\n\
        22 11 13  6  5\n\
         2  0 12  3  7\
    ";

    #[test]
    fn example_1_produces_4512() {
        let mut game: Game = EXAMPLE.parse().unwrap();
        assert_eq!(4512, game.play_to_win());
    }

    #[test]
    fn part_1_works() {
        let mut game: Game = parse_file("src/day04/input.txt");
        assert_eq!(8442, game.play_to_win());
    }

    #[test]
    fn example_2_produces_1924() {
        let mut game: Game = EXAMPLE.parse().unwrap();
        assert_eq!(1924, game.play_to_lose());
    }

    #[test]
    fn part_2_works() {
        let mut game: Game = parse_file("src/day04/input.txt");
        assert_eq!(4590, game.play_to_lose());
    }
}
