use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use amphipod::{Amphipod, AmphipodType, Path};
pub use burrow::Burrow;
use position::Position;

mod amphipod;
mod burrow;
mod position;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct BurrowState<const ROOM_SIZE: usize> {
    burrow: Burrow<ROOM_SIZE>,
    energy_spent: usize,
    min_total_energy: usize,
}

impl<const ROOM_SIZE: usize> BurrowState<ROOM_SIZE> {
    fn new(burrow: Burrow<ROOM_SIZE>) -> Self {
        Self {
            burrow,
            energy_spent: 0,
            min_total_energy: burrow.calc_min_energy_needed(),
        }
    }

    fn improve(&mut self) {
        while let Some(path) = self.burrow.find_path_to_room() {
            self.apply(path);
        }
    }

    fn apply(&mut self, path: Path) {
        self.energy_spent += path.energy_cost();
        self.min_total_energy += path.energy_wasted();
        self.burrow.apply(path);
    }

    fn new_states(&self) -> impl Iterator<Item = Self> + '_ {
        self.burrow.paths_to_hall().map(move |path| {
            let mut new = *self;
            new.apply(path);
            new
        })
    }
}

impl<const ROOM_SIZE: usize> Ord for BurrowState<ROOM_SIZE> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .min_total_energy
            .cmp(&self.min_total_energy)
            .then_with(|| other.energy_spent.cmp(&self.energy_spent))
            .then_with(|| other.burrow.cmp(&self.burrow))
    }
}

impl<const ROOM_SIZE: usize> PartialOrd for BurrowState<ROOM_SIZE> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_1<const ROOM_SIZE: usize>(burrow: Burrow<ROOM_SIZE>) -> usize {
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push(BurrowState::new(burrow));
    while let Some(mut state) = heap.pop() {
        if visited.insert(state.burrow) {
            state.improve();
            if state.energy_spent == state.min_total_energy {
                return state.energy_spent;
            }
            heap.extend(state.new_states());
        }
    }
    panic!("No solution!")
}

pub fn part_2(burrow: Burrow<2>) -> usize {
    let rooms = burrow.rooms;
    let unfolded_burrow: Burrow<4> = Burrow {
        hall: burrow.hall,
        rooms: [
            [
                rooms[0][0],
                Some(AmphipodType::Desert),
                Some(AmphipodType::Desert),
                rooms[0][1],
            ],
            [
                rooms[1][0],
                Some(AmphipodType::Copper),
                Some(AmphipodType::Bronze),
                rooms[1][1],
            ],
            [
                rooms[2][0],
                Some(AmphipodType::Bronze),
                Some(AmphipodType::Amber),
                rooms[2][1],
            ],
            [
                rooms[3][0],
                Some(AmphipodType::Amber),
                Some(AmphipodType::Copper),
                rooms[3][1],
            ],
        ],
    };
    part_1(unfolded_burrow)
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    use super::*;

    #[test]
    fn example_1_produces_12521() {
        let burrow = EXAMPLE.parse().unwrap();
        assert_eq!(12521, part_1(burrow));
    }

    #[test]
    fn part_1_works() {
        let burrow = parse_file("src/day23/input.txt");
        assert_eq!(14460, part_1(burrow));
    }

    #[test]
    fn example_2_produces_44169() {
        let burrow = EXAMPLE.parse().unwrap();
        assert_eq!(44169, part_2(burrow));
    }

    #[test]
    fn part_2_works() {
        let burrow = parse_file("src/day23/input.txt");
        assert_eq!(41366, part_2(burrow));
    }

    const EXAMPLE: &str = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
}
