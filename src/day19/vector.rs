use std::fmt;
use std::ops::{Add, Neg, Sub};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl Vector {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn distance(&self, other: Vector) -> i32 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }

    pub fn manhattan_distance(&self, other: Vector) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    pub fn orientation(self, i: u8) -> Vector {
        self.face(i % 6).rot_x((i / 6) % 4)
    }

    fn face(self, i: u8) -> Vector {
        let Self { x, y, z } = self;
        match i {
            0 => self,
            1 => Self::new(z, y, -x),
            2 => Self::new(-x, y, -z),
            3 => Self::new(-z, y, x),
            4 => Self::new(y, -x, z),
            5 => Self::new(-y, x, z),
            _ => panic!("Invalid face index"),
        }
    }

    fn rot_x(self, amount: u8) -> Vector {
        let Self { x, y, z } = self;
        match amount {
            0 => self,
            1 => Self::new(x, z, -y),
            2 => Self::new(x, -y, -z),
            3 => Self::new(x, -z, y),
            _ => panic!("Invalid rotation amount"),
        }
    }
}

impl FromStr for Vector {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut numbers = input.splitn(3, ',').map(|s| s.parse::<i32>().unwrap());
        let x = numbers.next().unwrap();
        let y = numbers.next().unwrap();
        let z = numbers.next().unwrap();
        assert_eq!(numbers.next(), None);
        Ok(Self { x, y, z })
    }
}
