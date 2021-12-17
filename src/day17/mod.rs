use std::ops::{Add, AddAssign};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash)]
pub struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy, Hash, Default)]
struct Vector {
    x: i32,
    y: i32,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl FromStr for TargetArea {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = &input.trim()[13..];
        let numbers: Vec<i32> = input
            .split(", ")
            .flat_map(|s| s[2..].split(".."))
            .map(|s| s.parse().unwrap())
            .collect();
        Ok(Self {
            x_min: numbers[0],
            x_max: numbers[1],
            y_min: numbers[2],
            y_max: numbers[3],
        })
    }
}

fn hits(v0: Vector, target: TargetArea) -> bool {
    let mut p = Vector::default();
    let mut v = v0;
    loop {
        p += v;
        if p.x > target.x_max || p.y < target.y_min {
            return false;
        }
        if p.x >= target.x_min && p.y <= target.y_max {
            return true;
        }
        if v.x > 0 {
            v.x -= 1;
        }
        v.y -= 1;
    }
}

#[allow(clippy::cast_possible_truncation)]
fn find_all_start_velocities(target: TargetArea) -> impl Iterator<Item = Vector> {
    let vx_min = ((f64::from(target.x_min).mul_add(8.0, 1.0).sqrt() - 1.0) / 2.0).ceil() as i32;
    let vy_max = -target.y_min; // no idea if this is correct, but it works
    (vx_min..=target.x_max)
        .flat_map(move |x| (target.y_min..=vy_max).map(move |y| Vector { x, y }))
        .filter(move |&v| hits(v, target))
}

pub fn part_1(target: TargetArea) -> i32 {
    let vy = find_all_start_velocities(target)
        .map(|v| v.y)
        .max()
        .unwrap();
    vy * (vy + 1) / 2
}

pub fn part_2(target: TargetArea) -> usize {
    find_all_start_velocities(target).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_file;

    const EXAMPLE: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn example_1_produces_45() {
        let target = EXAMPLE.parse().unwrap();
        assert_eq!(45, part_1(target));
    }

    #[test]
    fn part_1_works() {
        let target = parse_file("src/day17/input.txt");
        assert_eq!(4851, part_1(target));
    }

    #[test]
    fn example_2() {
        let target = EXAMPLE.parse().unwrap();
        assert_eq!(112, part_2(target));
    }

    #[test]
    fn part_2_works() {
        let target = parse_file("src/day17/input.txt");
        assert_eq!(1739, part_2(target));
    }
}
