#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum Position {
    SideRoom(usize, usize),
    Hallway(usize),
}

impl Position {
    pub const fn hallway_index(self) -> usize {
        match self {
            Self::SideRoom(room_nb, _) => hall_entrance_for_room(room_nb),
            Self::Hallway(index) => index,
        }
    }

    pub const fn distance_to_hall(self) -> usize {
        match self {
            Self::SideRoom(_, spot) => spot + 1,
            Self::Hallway(_) => 0,
        }
    }
}

pub const fn hall_entrance_for_room(room_nb: usize) -> usize {
    (room_nb + 1) * 2
}
