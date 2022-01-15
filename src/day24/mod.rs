use input::{Instruction, Value, Variable};
use std::mem;
use std::ops::{Index, IndexMut};

mod input;

#[derive(Clone, Debug, Default)]
struct Alu {
    w: Expression,
    x: Expression,
    y: Expression,
    z: Expression,
    input_index: u8,
}

impl Index<Variable> for Alu {
    type Output = Expression;

    fn index(&self, var: Variable) -> &Self::Output {
        match var {
            Variable::W => &self.w,
            Variable::X => &self.x,
            Variable::Y => &self.y,
            Variable::Z => &self.z,
        }
    }
}

impl IndexMut<Variable> for Alu {
    fn index_mut(&mut self, var: Variable) -> &mut Self::Output {
        match var {
            Variable::W => &mut self.w,
            Variable::X => &mut self.x,
            Variable::Y => &mut self.y,
            Variable::Z => &mut self.z,
        }
    }
}

impl Alu {
    pub fn process(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Inp(var) => self.set_input(var),
            Instruction::Add(var, val) => self.add(var, val),
            Instruction::Mul(var, val) => self.mul(var, val),
            Instruction::Div(var, val) => self.div(var, val),
            Instruction::Mod(var, val) => self.modulo(var, val),
            Instruction::Eql(var, val) => self.eql(var, val),
        }
    }

    fn get(&self, val: Value) -> Expression {
        match val {
            Value::Variable(var) => self[var].clone(),
            Value::Number(number) => Expression::Value(number),
        }
    }

    fn take_box(&mut self, var: Variable) -> Box<Expression> {
        Box::new(mem::take(&mut self[var]))
    }

    fn set_input(&mut self, var: Variable) {
        self[var] = Expression::Input {
            index: self.input_index,
        };
        self.input_index += 1;
    }

    fn add(&mut self, var: Variable, val: Value) {
        match (&self[var], self.get(val)) {
            (_, Expression::ZERO) => {}
            (&Expression::ZERO, val) => self[var] = val,
            (Expression::Value(left), Expression::Value(right)) => {
                self[var] = Expression::Value(left + right);
            }
            (_, val) => self[var] = Expression::Add(self.take_box(var), Box::new(val)),
        }
    }

    fn mul(&mut self, var: Variable, val: Value) {
        match (&self[var], self.get(val)) {
            (&Expression::ZERO, _) | (_, Expression::ONE) => {}
            (_, Expression::ZERO) => self[var] = Expression::ZERO,
            (&Expression::ONE, val) => self[var] = val,
            (Expression::Value(left), Expression::Value(right)) => {
                self[var] = Expression::Value(left * right);
            }
            (_, val) => self[var] = Expression::Mul(self.take_box(var), Box::new(val)),
        }
    }

    fn div(&mut self, var: Variable, val: Value) {
        match (&self[var], self.get(val)) {
            (_, Expression::ZERO) => panic!("Division by zero"),
            (&Expression::ZERO, _) | (_, Expression::ONE) => {}
            (Expression::Value(left), Expression::Value(right)) => {
                self[var] = Expression::Value(left / right);
            }
            (_, val) => self[var] = Expression::Div(self.take_box(var), Box::new(val)),
        }
    }

    fn modulo(&mut self, var: Variable, val: Value) {
        match (&self[var], self.get(val)) {
            (_, Expression::ZERO) => panic!("Modulo by zero"),
            (&Expression::ZERO, _) | (_, Expression::ONE) => {}
            (Expression::Value(left), Expression::Value(right)) => {
                self[var] = Expression::Value(left % right);
            }
            (_, val) => self[var] = Expression::Mod(self.take_box(var), Box::new(val)),
        }
    }


    fn eql(&mut self, var: Variable, val: Value) {
        match (&self[var], self.get(val)) {
            (Expression::Value(left), Expression::Value(right)) => {
                self[var] = Expression::Value(if *left == right { 1 } else { 0 });
            }
            (_, val) => self[var] = Expression::Eql(self.take_box(var), Box::new(val)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Expression {
    Value(i8),
    Input { index: u8 },
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Mod(Box<Expression>, Box<Expression>),
    Eql(Box<Expression>, Box<Expression>),
}

impl Expression {
    const ZERO: Expression = Expression::Value(0);
    const ONE: Expression = Expression::Value(1);
}

impl Default for Expression {
    fn default() -> Self {
        Expression::Value(0)
    }
}

pub fn part_1(instructions: impl Iterator<Item = Instruction>) -> u32 {
    let alu = instructions.fold(Alu::default(), |mut alu, instruction| {
        println!("{:?}", instruction);
        alu.process(instruction);
        alu
    });
    0
}

#[cfg(test)]
mod tests {
    use crate::{parse_file_lines, parse_str_lines};

    use super::*;

    #[test]
    fn part_1_works() {
        let instructions = parse_file_lines("src/day24/input.txt");
        assert_eq!(0, part_1(instructions));
    }
}
