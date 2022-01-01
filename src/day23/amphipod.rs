use crate::day23::position::hall_entrance_for_room;
use std::fmt;
use std::fmt::Write;

use super::Position;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct Amphipod {
    pub position: Position,
    pub type_: AmphipodType,
}

impl Amphipod {
    pub const fn energy_cost(&self) -> usize {
        self.type_.energy_cost()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct Path {
    pub amphipod: Amphipod,
    pub target: Position,
}

impl Path {
    pub const fn energy_cost(&self) -> usize {
        self.distance() * self.amphipod.energy_cost()
    }

    pub const fn energy_wasted(&self) -> usize {
        self.distance_wasted() * 2 * self.amphipod.energy_cost()
    }

    const fn distance_wasted(&self) -> usize {
        match (self.amphipod.position, self.target) {
            (Position::SideRoom(room_nb, _), Position::Hallway(to)) => {
                let from = hall_entrance_for_room(room_nb);
                let target = hall_entrance_for_room(self.amphipod.type_.target_room_nb());
                if from <= target && target <= to {
                    to - target
                } else if to <= from && from <= target {
                    from - to
                } else if to <= target && target <= from {
                    target - to
                } else if target <= from && from <= to {
                    to - from
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    pub const fn distance(&self) -> usize {
        self.amphipod.position.distance_to_hall()
            + self.target.distance_to_hall()
            + delta(
                self.amphipod.position.hallway_index(),
                self.target.hallway_index(),
            )
    }
}

const fn delta(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, PartialOrd, Ord)]
pub enum AmphipodType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl fmt::Display for AmphipodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            AmphipodType::Amber => 'A',
            AmphipodType::Bronze => 'B',
            AmphipodType::Copper => 'C',
            AmphipodType::Desert => 'D',
        })
    }
}

impl AmphipodType {
    pub const fn target_room_nb(self) -> usize {
        match self {
            Self::Amber => 0,
            Self::Bronze => 1,
            Self::Copper => 2,
            Self::Desert => 3,
        }
    }

    pub const fn energy_cost(self) -> usize {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }

    pub fn option_from(input: char) -> Option<Self> {
        if input == '.' {
            None
        } else {
            Some(Self::from(input))
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<char> for AmphipodType {
    fn from(input: char) -> Self {
        match input {
            'A' => Self::Amber,
            'B' => Self::Bronze,
            'C' => Self::Copper,
            'D' => Self::Desert,
            _ => panic!("Invalid Amphipod: {}", input),
        }
    }
}
