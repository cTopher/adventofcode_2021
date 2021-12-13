use super::{Dot, Fold};
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Formatter, Write};
use std::str::FromStr;

#[derive(Default, Clone)]
pub struct Paper {
    dots: HashSet<Dot>,
    instructions: Vec<Fold>,
}

impl FromStr for Paper {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (dots, instructions) = input.split_once("\n\n").unwrap();
        Ok(Self {
            dots: dots.lines().map(|line| line.parse().unwrap()).collect(),
            instructions: instructions
                .lines()
                .map(|line| line.parse().unwrap())
                .collect(),
        })
    }
}

impl Paper {
    pub fn do_first_instruction(&mut self) {
        Self::do_instruction(&mut self.dots, self.instructions[0]);
    }

    pub fn do_instructions(&mut self) {
        for &instruction in &self.instructions {
            Self::do_instruction(&mut self.dots, instruction);
        }
    }

    pub fn number_of_dots(&self) -> usize {
        self.dots.len()
    }

    fn do_instruction(dots: &mut HashSet<Dot>, instruction: Fold) {
        *dots = dots.drain().map(|dot| dot.fold(instruction)).collect();
    }

    fn width(&self) -> u16 {
        self.instructions
            .iter()
            .filter_map(|&instruction| match instruction {
                Fold::X(x) => Some(x),
                Fold::Y(_) => None,
            })
            .min()
            .unwrap()
    }

    fn height(&self) -> u16 {
        self.instructions
            .iter()
            .filter_map(|&instruction| match instruction {
                Fold::X(_) => None,
                Fold::Y(y) => Some(y),
            })
            .min()
            .unwrap()
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.dots.contains(&Dot { x, y }) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
