use std::str::FromStr;

#[derive(Debug)]
pub struct Board {
    grid: [[Cell; 5]; 5],
    victory: bool,
}

#[derive(Debug)]
pub struct Cell {
    number: u8,
    marked: bool,
}

impl Cell {
    pub const fn marked(&self) -> bool {
        self.marked
    }
}

impl FromStr for Cell {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            number: input.parse().unwrap(),
            marked: false,
        })
    }
}

impl Board {
    fn indexes() -> impl Iterator<Item = (usize, usize)> {
        (0..5).flat_map(|i| (0..5).map(move |j| (i, j)))
    }

    fn cells(&self) -> impl Iterator<Item = &Cell> {
        Self::indexes().map(move |(i, j)| &self.grid[i][j])
    }

    pub const fn victory(&self) -> bool {
        self.victory
    }

    fn position(&self, number: u8) -> Option<(usize, usize)> {
        Self::indexes().find(|&(i, j)| self.grid[i][j].number == number)
    }

    pub fn apply(&mut self, number: u8) {
        if let Some((m, n)) = self.position(number) {
            self.grid[m][n].marked = true;
            if self.row(m).all(Cell::marked) || self.column(n).all(Cell::marked) {
                self.victory = true;
            }
        }
    }

    fn row(&self, i: usize) -> impl Iterator<Item = &Cell> {
        (0..5).map(move |j| &self.grid[i][j])
    }

    fn column(&self, j: usize) -> impl Iterator<Item = &Cell> {
        (0..5).map(move |i| &self.grid[i][j])
    }

    pub fn score(&self, multiplier: u32) -> u32 {
        let unmarked: u32 = self
            .cells()
            .filter(|&cell| !cell.marked)
            .map(|cell| u32::from(cell.number))
            .sum();
        unmarked * multiplier
    }
}

macro_rules! arr_5 {
    ($x:expr) => {
        [$x, $x, $x, $x, $x]
    };
}

impl FromStr for Board {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let row = |input: &str| -> [Cell; 5] {
            let mut numbers = input.split(' ').filter(|s| !s.is_empty());
            arr_5!(numbers.next().unwrap().parse().unwrap())
        };
        let mut lines = input.lines();
        Ok(Self {
            grid: arr_5!(row(lines.next().unwrap())),
            victory: false,
        })
    }
}
