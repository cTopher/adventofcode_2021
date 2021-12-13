use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Copy)]
pub enum Fold {
    X(u16),
    Y(u16),
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let n = input.split_once('=').unwrap().1.parse().unwrap();
        Ok(if input.starts_with("fold along x") {
            Self::X(n)
        } else if input.starts_with("fold along y") {
            Self::Y(n)
        } else {
            panic!("Unknown fold instruction: {}", input)
        })
    }
}
