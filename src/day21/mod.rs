use std::cmp::{max, min};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Game {
    player_1: Player,
    player_2: Player,
    turn: usize,
}

#[derive(Clone, Debug, Copy)]
struct Player {
    score: usize,
    position: usize,
}

impl Game {
    fn play_practice(&mut self, dice: &mut DeterministicDice) {
        loop {
            let player = if self.turn == 0 {
                &mut self.player_1
            } else {
                &mut self.player_2
            };
            player.take_practice_turn(dice);
            if player.score >= 1000 {
                return;
            }
            self.turn = (self.turn + 1) % 2;
        }
    }

    fn take_quantum_turn(self) -> impl Iterator<Item = Self> {
        let (active_player, passive_player) = if self.turn == 0 {
            (self.player_1, self.player_2)
        } else {
            (self.player_2, self.player_1)
        };
        active_player.take_quantum_turn().map(move |active_player| {
            let (player_1, player_2) = if self.turn == 0 {
                (active_player, passive_player)
            } else {
                (passive_player, active_player)
            };
            Self {
                player_1,
                player_2,
                turn: (self.turn + 1) % 2,
            }
        })
    }
}

impl From<Game> for usize {
    fn from(game:Game) -> Self {
        let mut result = game.player_1.score;
        result =result * 1000 + game.player_2.score;
        result = result * 101 + game.player_1.position;
        result = result * 101 + game.player_2.position;
        result = result * 2 + game.turn;
        result
    }
}

impl From<usize> for Game {
    fn from(input: usize) -> Self {
        let turn = input % 2;
        let input = input / 2;
        let player_2_position = input % 101;
        let input = input / 101;
        let player_1_position = input % 101;
        let input = input / 101;
        let player_2_score = input % 1000;
        let input = input / 1000;
        let player_1_score = input;
        Self {
            player_1: Player {
                score: player_1_score,
                position: player_1_position,
            },
            player_2: Player {
                score: player_2_score,
                position: player_2_position,
            },
            turn,
        }
    }
}



impl Player {
    fn take_practice_turn(&mut self, dice: &mut DeterministicDice) {
        let roll = dice.roll() + dice.roll() + dice.roll();
        self.position = (self.position - 1 + roll) % 10 + 1;
        self.score += self.position;
    }

    fn take_quantum_turn(self) -> impl Iterator<Item = Self> {
        roll_quantum_dice().map(move |roll| {
            let position = (self.position - 1 + roll) % 10 + 1;
            let score = self.score + position;
            Self {score, position }
        })
    }
}

fn roll_quantum_dice() -> impl Iterator<Item = usize> {
    (1..=3).flat_map(|a| (1..=3).flat_map(move |b| (1..=3).map(move |c| a + b + c)))
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
struct DeterministicDice {
    rolls: usize,
}

impl DeterministicDice {
    fn new() -> Self {
        Self::default()
    }

    fn roll(&mut self) -> usize {
        let result = self.rolls % 100 + 1;
        self.rolls += 1;
        result
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();
        Ok(Self {
            player_1: lines.next().unwrap().parse().unwrap(),
            player_2: lines.next().unwrap().parse().unwrap(),
            turn: 0,
        })
    }
}

impl FromStr for Player {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let score = 0;
        let position = input[28..].parse().unwrap();
        Ok(Self { score, position })
    }
}

pub fn part_1(game: &mut Game) -> usize {
    let mut dice = DeterministicDice::new();
    game.play_practice(&mut dice);
    min(game.player_1.score, game.player_2.score) * dice.rolls
}

pub fn part_2(game: Game) -> u64 {
    let mut games:Vec<u64> = vec![0;1000*1000*101*101*2];
    games[usize::from(game)] = 1;
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    for index in 0..games.len() {
        let count = games[index];
        if count == 0 {
            continue;
        }
        let game = Game::from(index);
        for game in game.take_quantum_turn() {
            if game.player_1.score >= 1000 {
                p1_wins += count;
            } else if game.player_2.score >= 1000 {
                p2_wins += count;
            } else {
                games[usize::from(game)] += count;
            }
        }
    }
    max(p1_wins, p2_wins)
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    use super::*;

    const EXAMPLE: &str = "\
        Player 1 starting position: 4\n\
        Player 2 starting position: 8\
    ";

    #[test]
    fn example_1_produces_739785() {
        let mut game = EXAMPLE.parse().unwrap();
        assert_eq!(739_785, part_1(&mut game));
    }

    #[test]
    fn part_1_works() {
        let mut game = parse_file("src/day21/input.txt");
        assert_eq!(903_630, part_1(&mut game));
    }

    #[test]
    fn example_2_produces_444356092776315() {
        let game = EXAMPLE.parse().unwrap();
        assert_eq!(444_356_092_776_315, part_2(game));
    }
}
