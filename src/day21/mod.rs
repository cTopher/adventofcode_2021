use std::cmp::{max, min, Ordering};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct Game {
    player_1: Player,
    player_2: Player,
    turn: bool,
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
struct Player {
    score: usize,
    position: usize,
}

impl From<Game> for usize {
    /// assumes scores < 21 (this would not work for part 1)
    fn from(game: Game) -> Self {
        let result = game.player_1.score;
        let result = result * 21 + game.player_2.score;
        let result = result * 11 + game.player_1.position;
        let result = result * 11 + game.player_2.position;
        result * 2 + game.turn as Self
    }
}

impl From<usize> for Game {
    fn from(input: usize) -> Self {
        let turn = input % 2 == 1;
        let input = input / 2;
        let player_2_position = input % 11;
        let input = input / 11;
        let player_1_position = input % 11;
        let input = input / 11;
        let player_2_score = input % 21;
        let input = input / 21;
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

const TRIPLE_QUANTUM: [(usize, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

impl Game {
    fn play_practice(&mut self, dice: &mut DeterministicDice) {
        loop {
            let player = if self.turn {
                &mut self.player_1
            } else {
                &mut self.player_2
            };
            player.take_practice_turn(dice);
            if player.score >= 1000 {
                return;
            }
            self.turn = !self.turn;
        }
    }

    fn take_quantum_turn(self) -> impl Iterator<Item = (Self, u64)> {
        let (active_player, passive_player) = if self.turn {
            (self.player_1, self.player_2)
        } else {
            (self.player_2, self.player_1)
        };
        active_player
            .take_quantum_turn()
            .map(move |(active_player, universes)| {
                let (player_1, player_2) = if self.turn {
                    (active_player, passive_player)
                } else {
                    (passive_player, active_player)
                };
                let new_game = Self {
                    player_1,
                    player_2,
                    turn: !self.turn,
                };
                (new_game, universes)
            })
    }
}

impl Player {
    fn take_practice_turn(&mut self, dice: &mut DeterministicDice) {
        let roll = dice.roll() + dice.roll() + dice.roll();
        self.position = (self.position - 1 + roll) % 10 + 1;
        self.score += self.position;
    }

    fn take_quantum_turn(self) -> impl Iterator<Item = (Self, u64)> {
        TRIPLE_QUANTUM.iter().map(move |&(roll, universes)| {
            let position = (self.position - 1 + roll) % 10 + 1;
            let score = self.score + position;
            (Self { score, position }, universes)
        })
    }
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
            turn: true,
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

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.player_1.score + self.player_2.score;
        let b = other.player_1.score + other.player_2.score;
        a.cmp(&b)
            .then_with(|| self.player_1.score.cmp(&other.player_1.score))
            .then_with(|| self.player_1.position.cmp(&other.player_1.position))
            .then_with(|| self.player_2.position.cmp(&other.player_2.position))
            .then_with(|| self.turn.cmp(&other.turn))
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_2(game: Game) -> u64 {
    let mut games: Vec<u64> = vec![0; 21 * 21 * 11 * 11 * 2];
    games[usize::from(game)] = 1;
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    for index in 0..games.len() {
        let count = games[index];
        if count == 0 {
            continue;
        }
        let game = Game::from(index);
        for (game, universes) in game.take_quantum_turn() {
            let universes = count * universes;
            if game.player_1.score >= 21 {
                p1_wins += universes;
            } else if game.player_2.score >= 21 {
                p2_wins += universes;
            } else {
                games[usize::from(game)] += universes;
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

    #[test]
    fn part_2_works() {
        let game = parse_file("src/day21/input.txt");
        assert_eq!(303_121_579_983_974, part_2(game));
    }
}
