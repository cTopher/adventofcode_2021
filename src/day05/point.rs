use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (x, y) = input.split_once(',').unwrap();
        Ok(Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}
