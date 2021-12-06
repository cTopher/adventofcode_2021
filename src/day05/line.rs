use std::cmp::max;
use std::str::FromStr;

use super::Point;

#[derive(Copy, Clone, Debug)]
pub struct Line {
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    size: usize,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (start, end) = input.split_once(" -> ").unwrap();
        Ok(Self::new(start.parse().unwrap(), end.parse().unwrap()))
    }
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        let dx = isize::try_from(end.x).unwrap() - isize::try_from(start.x).unwrap();
        let dy = isize::try_from(end.y).unwrap() - isize::try_from(start.y).unwrap();
        Self {
            x: start.x,
            y: start.y,
            dx: normalize(dx),
            dy: normalize(dy),
            size: usize::try_from(max(dx.abs(), dy.abs())).unwrap(),
        }
    }

    pub const fn horizontal(&self) -> bool {
        self.dy == 0
    }

    pub const fn vertical(&self) -> bool {
        self.dx == 0
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..=(self.size as isize)).map(move |i| Point {
            x: (self.x as isize + (self.dx * i)) as usize,
            y: (self.y as isize + (self.dy * i)) as usize,
        })
    }
}

const fn normalize(a: isize) -> isize {
    match a {
        0 => 0,
        a if a < 0 => -1,
        _ => 1,
    }
}
