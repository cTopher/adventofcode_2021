use super::{Amphipod, AmphipodType, Path};
use crate::day23::position::{hall_entrance_for_room, Position};
use std::fmt;
use std::fmt::{Formatter, Write};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, PartialOrd, Ord)]
pub struct Hallway([Option<AmphipodType>; 11]);

impl Hallway {
    fn amphipods(&self) -> impl Iterator<Item = Amphipod> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter_map(move |(index, &type_)| {
                type_.map(|type_| Amphipod {
                    position: Position::Hallway(index),
                    type_,
                })
            })
    }

    /// from exclusive, to inclusive
    pub fn is_clear(&self, from: usize, to: usize) -> bool {
        let slice = if from < to {
            &self.0[from + 1..=to]
        } else {
            &self.0[to..from]
        };
        slice.iter().all(|&a| a.is_none())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct SideRoom<'a> {
    number: usize,
    room: &'a [Option<AmphipodType>],
}

impl<'a> SideRoom<'a> {
    pub fn movable_amphipod(&self) -> Option<Amphipod> {
        if self.contains_only(self.target_amphipod_type()) {
            None
        } else {
            self.room
                .iter()
                .enumerate()
                .find_map(move |(index, &type_)| {
                    type_.map(|type_| Amphipod {
                        position: Position::SideRoom(self.number, index),
                        type_,
                    })
                })
        }
    }

    pub fn contains_only(&self, amphipod: AmphipodType) -> bool {
        self.room
            .iter()
            .all(|a| a.is_none() || a == &Some(amphipod))
    }

    pub fn last_empty_spot(&self) -> Option<usize> {
        self.room.iter().rposition(Option::is_none)
    }

    pub fn wrongly_placed_amphipods(&self) -> impl Iterator<Item = Amphipod> + '_ {
        let target_type = Some(self.target_amphipod_type());
        self.room
            .iter()
            .enumerate()
            .rev()
            .skip_while(move |&(_, type_)| type_ == &target_type)
            .filter_map(move |(index, &type_)| {
                type_.map(move |type_| Amphipod {
                    position: Position::SideRoom(self.number, index),
                    type_,
                })
            })
    }

    pub fn spots_to_fill(&self) -> usize {
        let target_type = Some(self.target_amphipod_type());
        self.room
            .iter()
            .rev()
            .skip_while(|&x| x == &target_type)
            .count()
    }

    pub fn target_amphipod_type(&self) -> AmphipodType {
        match self.number {
            0 => AmphipodType::Amber,
            1 => AmphipodType::Bronze,
            2 => AmphipodType::Copper,
            3 => AmphipodType::Desert,
            _ => panic!("Invalid room number"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Ord, PartialOrd)]
pub struct Burrow<const ROOM_SIZE: usize> {
    pub hall: Hallway,
    pub rooms: [[Option<AmphipodType>; ROOM_SIZE]; 4],
}

impl<const ROOM_SIZE: usize> Burrow<ROOM_SIZE> {
    fn side_rooms(&self) -> impl Iterator<Item = SideRoom> {
        self.rooms
            .iter()
            .enumerate()
            .map(move |(number, room)| SideRoom { number, room })
    }

    const fn side_room(&self, number: usize) -> SideRoom {
        let room = &self.rooms[number];
        SideRoom { number, room }
    }

    fn movable_amphipods(&self) -> impl Iterator<Item = Amphipod> + '_ {
        self.hall.amphipods().chain(self.movable_room_amphipods())
    }

    fn movable_room_amphipods(&self) -> impl Iterator<Item = Amphipod> + '_ {
        self.side_rooms().filter_map(|room| room.movable_amphipod())
    }

    pub fn find_path_to_room(&self) -> Option<Path> {
        self.movable_amphipods().find_map(|amphipod| {
            let target_room_nb = amphipod.type_.target_room_nb();
            let room = self.side_room(target_room_nb);
            let hall_clear = self.hall.is_clear(
                amphipod.position.hallway_index(),
                hall_entrance_for_room(target_room_nb),
            );
            if hall_clear && room.contains_only(amphipod.type_) {
                Some(Path {
                    amphipod,
                    target: Position::SideRoom(target_room_nb, room.last_empty_spot().unwrap()),
                })
            } else {
                None
            }
        })
    }

    pub fn calc_min_energy_needed(&self) -> usize {
        let cost_to_move_to_rooms: usize = self
            .hall
            .amphipods()
            .chain(
                self.side_rooms()
                    .flat_map(|room| room.wrongly_placed_amphipods().collect::<Vec<Amphipod>>()),
            )
            .map(|amphipod| {
                Path {
                    amphipod,
                    target: Position::SideRoom(amphipod.type_.target_room_nb(), 0),
                }
                .energy_cost()
            })
            .sum();
        let cost_to_move_within_target_room: usize = self
            .side_rooms()
            .map(|room| {
                let spots = room.spots_to_fill();
                if spots > 1 {
                    room.target_amphipod_type().energy_cost() * (spots * (spots - 1) / 2)
                } else {
                    0
                }
            })
            .sum();
        cost_to_move_to_rooms + cost_to_move_within_target_room
    }

    pub fn paths_to_hall(&self) -> impl Iterator<Item = Path> + '_ {
        self.movable_room_amphipods().flat_map(move |amphipod| {
            let from = amphipod.position.hallway_index();
            VALID_HALL_POSITIONS.iter().filter_map(move |&target| {
                if self.hall.is_clear(from, target) {
                    Some(Path {
                        amphipod,
                        target: Position::Hallway(target),
                    })
                } else {
                    None
                }
            })
        })
    }

    pub fn apply(&mut self, path: Path) {
        let removed = self.get_mut(path.amphipod.position).take();
        assert_eq!(removed, Some(path.amphipod.type_));
        let replaced = self.get_mut(path.target).replace(path.amphipod.type_);
        assert_eq!(replaced, None);
    }

    fn get_mut(&mut self, pos: Position) -> &mut Option<AmphipodType> {
        match pos {
            Position::SideRoom(number, spot) => &mut self.rooms[number][spot],
            Position::Hallway(pos) => &mut self.hall.0[pos],
        }
    }
}

impl FromStr for Burrow<2> {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines().skip(1);
        let hall: Vec<_> = lines
            .next()
            .unwrap()
            .chars()
            .skip(1)
            .take(11)
            .map(AmphipodType::option_from)
            .collect();
        let first: Vec<char> = lines.next().unwrap().chars().collect();
        let second: Vec<char> = lines.next().unwrap().chars().collect();
        let room = |index: usize| {
            let index = index * 2 + 3;
            [
                AmphipodType::option_from(first[index]),
                AmphipodType::option_from(second[index]),
            ]
        };
        Ok(Self {
            rooms: [room(0), room(1), room(2), room(3)],
            hall: Hallway([
                hall[0], hall[1], hall[2], hall[3], hall[4], hall[5], hall[6], hall[7], hall[8],
                hall[9], hall[10],
            ]),
        })
    }
}

impl<const ROOM_SIZE: usize> fmt::Display for Burrow<ROOM_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // above hall
        writeln!(f, "#############")?;
        // hall
        f.write_char('#')?;
        for amphipod in self.hall.0 {
            fmt_amiphod(amphipod, f)?;
        }
        writeln!(f, "#")?;
        // room_top
        write!(f, "###")?;
        for room in self.rooms {
            fmt_amiphod(room[0], f)?;
            f.write_char('#')?;
        }
        writeln!(f, "##")?;
        // room_bottom
        write!(f, "  #")?;
        for room in self.rooms {
            fmt_amiphod(room[1], f)?;
            f.write_char('#')?;
        }
        writeln!(f)?;
        // below room
        writeln!(f, "  #########")
    }
}

fn fmt_amiphod(amphipod: Option<AmphipodType>, f: &mut Formatter<'_>) -> fmt::Result {
    if let Some(amphipod) = amphipod {
        write!(f, "{}", amphipod)
    } else {
        f.write_char('.')
    }
}

const VALID_HALL_POSITIONS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
