use std::str::FromStr;

use super::Fold;

#[derive(Default, Clone, Eq, PartialEq, Hash, Debug, Copy)]
pub struct Dot {
    pub x: u16,
    pub y: u16,
}

impl Dot {
    pub const fn fold(self, fold: Fold) -> Self {
        match fold {
            Fold::X(n) if self.x > n => Self {
                x: 2 * n - self.x,
                y: self.y,
            },
            Fold::Y(n) if self.y > n => Self {
                x: self.x,
                y: 2 * n - self.y,
            },
            _ => self,
        }
    }
}

impl FromStr for Dot {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (x, y) = input.split_once(',').unwrap();
        Ok(Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}
