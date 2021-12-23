use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::{Formatter, Write};
use std::str::FromStr;
use std::{fmt, mem};

type Hall = [Option<Amphipod>; 11];
type Room = [Option<Amphipod>; 2];

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct Burrow {
    hall: Hall,
    rooms: [Room; 4],
    energy_spent: usize,
    min_total_energy: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

const AMPHIPODS: [Amphipod; 4] = [
    Amphipod::Amber,
    Amphipod::Bronze,
    Amphipod::Copper,
    Amphipod::Desert,
];

impl Amphipod {
    fn target_room(&self) -> usize {
        match self {
            Amphipod::Amber => 0,
            Amphipod::Bronze => 1,
            Amphipod::Copper => 2,
            Amphipod::Desert => 3,
        }
    }

    fn energy(&self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }

    fn target_room_entrance(&self) -> usize {
        hall_entrance_for_room(self.target_room())
    }

    fn can_enter(self, room: Room) -> bool {
        room[0].is_none()
            && match room[1] {
                None => true,
                Some(x) => x == self,
            }
    }

    fn optional_from(input: char) -> Option<Amphipod> {
        if input == '.' {
            None
        } else {
            Some(Amphipod::from(input))
        }
    }

    fn min_energy_from_room(self, room: usize, position: usize) -> usize {
        let target = self.target_room();
        if room == target {
            return 0;
        }
        (2 * delta(room, target) + 2 + position) * self.energy()
    }

    fn energy_from_hall(self, position: usize) -> usize {
        (delta(self.target_room_entrance(), position) + 1) * self.energy()
    }
}

const VALID_HALL_POSITIONS: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];

fn hall_entrance_for_room(room: usize) -> usize {
    (room + 1) * 2
}

impl Burrow {
    //TODO merge move_room_to_room_states and move_room_to_hall_states
    fn new_states(&self) -> impl Iterator<Item = Self> + '_ {
        // println!("FROM");
        // println!("{}", self);
        // println!("TO");
        // let e = self.energy_spent;
        self.move_room_to_room_states()
            .chain(self.move_hall_to_room_states())
            .chain(self.move_room_to_hall_states())
            .map(|mut burrow| {
                // println!("{}", burrow.energy_spent - e);
                // println!("{}", burrow);
                burrow.update_min_total_energy();
                burrow
            })
    }

    fn move_room_to_room_states(&self) -> impl Iterator<Item = Self> + '_ {
        (0..4).filter_map(|room_index| {
            let (burrow, hall_index) = self.move_out_of_room(room_index)?;
            burrow.move_hall_to_room(hall_index)
        })
    }

    fn move_hall_to_room_states(&self) -> impl Iterator<Item = Self> + '_ {
        (0..11).filter_map(|hall_position| self.move_hall_to_room(hall_position))
    }

    fn move_room_to_hall_states(&self) -> impl Iterator<Item = Self> + '_ {
        (0..4)
            .filter_map(|room_index| self.move_out_of_room(room_index))
            .flat_map(|(burrow, hall_position)| {
                VALID_HALL_POSITIONS.iter().filter_map(move |&target| {
                    let mut burrow = burrow;
                    if !burrow.hallway_is_clear(target, hall_position) {
                        return None;
                    }
                    let amiphod = mem::take(&mut burrow.hall[hall_position]);
                    burrow.hall[target] = amiphod;
                    burrow.energy_spent += delta(hall_position, target) * amiphod.unwrap().energy();
                    Some(burrow)
                })
            })
    }

    fn move_out_of_room(mut self, room_index: usize) -> Option<(Self, usize)> {
        let hall_index = hall_entrance_for_room(room_index);
        let room = &mut self.rooms[room_index];
        let amiphod = match (room[0], room[1]) {
            (Some(amiphod), _) => {
                if amiphod.target_room() == room_index && room[1] == Some(amiphod) {
                    return None;
                }
                room[0] = None;
                amiphod
            }
            (None, Some(amiphod)) => {
                if amiphod.target_room() == room_index {
                    return None;
                }
                self.energy_spent += amiphod.energy();
                room[1] = None;
                amiphod
            }
            _ => return None,
        };
        self.hall[hall_index] = Some(amiphod);
        self.energy_spent += amiphod.energy();
        Some((self, hall_index))
    }

    fn move_hall_to_room(mut self, hall_position: usize) -> Option<Self> {
        let amiphod = mem::take(&mut self.hall[hall_position])?;
        let target_room_entrance = amiphod.target_room_entrance();
        if !self.hallway_is_clear(target_room_entrance, hall_position) {
            return None;
        }
        if !amiphod.can_enter(self.rooms[amiphod.target_room()]) {
            return None;
        }
        self.energy_spent += amiphod.energy_from_hall(hall_position);
        let room = &mut self.rooms[amiphod.target_room()];
        if room[1].is_none() {
            self.energy_spent += amiphod.energy();
            room[1] = Some(amiphod);
        } else {
            room[0] = Some(amiphod);
        }
        Some(self)
    }

    fn hallway_is_clear(&self, from: usize, to: usize) -> bool {
        if from < to {
            (from..to).all(|i| self.hall[i].is_none())
        } else {
            (to + 1..=from).all(|i| self.hall[i].is_none())
        }
    }

    fn update_min_total_energy(&mut self) {
        self.min_total_energy = self.energy_spent;
        for (room_index, room) in self.rooms.iter().enumerate() {
            for (position, amphipod) in room.iter().enumerate() {
                if let Some(amiphod) = amphipod {
                    self.min_total_energy += amiphod.min_energy_from_room(room_index, position);
                }
            }
        }
        for (position, amphipod) in self.hall.iter().enumerate() {
            if let Some(amphipod) = amphipod {
                let i = amphipod.energy_from_hall(position);
                self.min_total_energy += i;
            }
        }
        for amphipod in AMPHIPODS {
            if self.rooms[amphipod.target_room()][1] != Some(amphipod) {
                self.min_total_energy += amphipod.energy();
            }
        }
    }
}

impl Ord for Burrow {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .min_total_energy
            .cmp(&self.min_total_energy)
            .then_with(|| other.energy_spent.cmp(&self.energy_spent))
    }
}

impl PartialOrd for Burrow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn delta(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}

impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            Amphipod::Amber => 'A',
            Amphipod::Bronze => 'B',
            Amphipod::Copper => 'C',
            Amphipod::Desert => 'D',
        })
    }
}

impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // above hall
        writeln!(f, "#############")?;
        // hall
        f.write_char('#')?;
        for amphipod in self.hall {
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

fn fmt_amiphod(amphipod: Option<Amphipod>, f: &mut Formatter<'_>) -> fmt::Result {
    if let Some(amphipod) = amphipod {
        write!(f, "{}", amphipod)
    } else {
        f.write_char('.')
    }
}

impl From<char> for Amphipod {
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

impl FromStr for Burrow {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines().skip(1);
        let hall: Vec<_> = lines
            .next()
            .unwrap()
            .chars()
            .skip(1)
            .take(11)
            .map(Amphipod::optional_from)
            .collect();
        let first: Vec<char> = lines.next().unwrap().chars().collect();
        let second: Vec<char> = lines.next().unwrap().chars().collect();
        let room = |index: usize| -> Room {
            let index = index * 2 + 3;
            [
                Amphipod::optional_from(first[index]),
                Amphipod::optional_from(second[index]),
            ]
        };
        let mut burrow = Self {
            rooms: [room(0), room(1), room(2), room(3)],
            hall: [
                hall[0], hall[1], hall[2], hall[3], hall[4], hall[5], hall[6], hall[7], hall[8],
                hall[9], hall[10],
            ],
            energy_spent: 0,
            min_total_energy: 0,
        };
        burrow.update_min_total_energy();
        Ok(burrow)
    }
}

pub fn part_1(burrow: Burrow) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push(burrow);
    while let Some(burrow) = heap.pop() {
        if burrow.energy_spent == burrow.min_total_energy {
            return burrow.energy_spent;
        }
        heap.extend(burrow.new_states());
    }
    panic!("No solution!")
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    use super::*;

    #[test]
    fn xxx() {
        let burrow: Burrow = "\
#############
#.....D.D...#
###.#B#C#.###
  #A#B#C#A#
  #########"
            .parse()
            .unwrap();
        println!("{}", burrow.min_total_energy);
        // 7011
        println!("{}", part_1(burrow));
    }

    #[test]
    fn example_1_produces_12521() {
        let burrow: Burrow = EXAMPLE.parse().unwrap();
        assert_eq!(12521, part_1(burrow));
    }

    #[test]
    fn part_1_works() {
        let burrow = parse_file("src/day23/input.txt");
        assert_eq!(14460, part_1(burrow));
    }

    const EXAMPLE: &str = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
}
