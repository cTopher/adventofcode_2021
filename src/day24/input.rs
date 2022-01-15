use std::str::FromStr;

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub enum Variable {
    W,
    X,
    Y,
    Z,
}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub enum Value {
    Variable(Variable),
    Number(i8),
}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub enum Instruction {
    Inp(Variable),
    Add(Variable, Value),
    Mul(Variable, Value),
    Div(Variable, Value),
    Mod(Variable, Value),
    Eql(Variable, Value),
}

impl FromStr for Value {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input.parse() {
            Ok(number) => Self::Number(number),
            Err(_) => Self::Variable(input.parse().unwrap()),
        })
    }
}

impl FromStr for Variable {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Variable::W),
            "x" => Ok(Variable::X),
            "y" => Ok(Variable::Y),
            "z" => Ok(Variable::Z),
            _ => panic!("Invalid variable: {}", s),
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split_whitespace();
        let instruction = parts.next().unwrap();
        let variable = parts.next().unwrap().parse().unwrap();
        let value = parts.next().map(|v| v.parse().unwrap());
        Ok(match instruction {
            "inp" => Self::Inp(variable),
            "add" => Self::Add(variable, value.unwrap()),
            "mul" => Self::Mul(variable, value.unwrap()),
            "div" => Self::Div(variable, value.unwrap()),
            "mod" => Self::Mod(variable, value.unwrap()),
            "eql" => Self::Eql(variable, value.unwrap()),
            _ => panic!("Invalid instruction: {}", instruction),
        })
    }
}
