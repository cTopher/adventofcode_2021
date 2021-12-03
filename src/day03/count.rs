use super::Bits;
use std::ops::Add;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitCounts {
    pub counts: Vec<u16>,
    pub size: u16,
}

impl From<Bits> for BitCounts {
    fn from(bits: Bits) -> Self {
        Self {
            counts: bits.into_iter().map(|b| b as u16).collect(),
            size: 1,
        }
    }
}

impl Add<Bits> for BitCounts {
    type Output = Self;

    fn add(self, bits: Bits) -> Self {
        Self {
            counts: self
                .counts
                .into_iter()
                .zip(bits)
                .map(|(count, bit)| count + bit as u16)
                .collect(),
            size: self.size + 1,
        }
    }
}
