use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (cmd, arg) = input.split_once(' ').ok_or(())?;
        match cmd {
            "forward" => Ok(Self::Forward(arg.parse().unwrap())),
            "down" => Ok(Self::Down(arg.parse().unwrap())),
            "up" => Ok(Self::Up(arg.parse().unwrap())),
            _ => Err(()),
        }
    }
}
