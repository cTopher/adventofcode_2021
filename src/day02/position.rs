use super::Command;

#[derive(Default, Copy, Clone, Debug)]
pub struct NaivePosition {
    pub horizontal: u32,
    pub depth: u32,
}

impl NaivePosition {
    pub const fn apply(self, cmd: Command) -> Self {
        let NaivePosition { horizontal, depth } = self;
        match cmd {
            Command::Forward(x) => Self {
                horizontal: horizontal + x,
                ..self
            },
            Command::Down(x) => Self {
                depth: depth + x,
                ..self
            },
            Command::Up(x) => Self {
                depth: depth - x,
                ..self
            },
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct FullPosition {
    pub horizontal: u32,
    pub depth: u32,
    aim: u32,
}

impl FullPosition {
    pub const fn apply(self, cmd: Command) -> Self {
        let FullPosition {
            horizontal,
            depth,
            aim,
        } = self;
        match cmd {
            Command::Down(x) => Self {
                aim: aim + x,
                ..self
            },
            Command::Up(x) => Self {
                aim: aim - x,
                ..self
            },
            Command::Forward(x) => Self {
                horizontal: horizontal + x,
                depth: depth + aim * x,
                ..self
            },
        }
    }
}
