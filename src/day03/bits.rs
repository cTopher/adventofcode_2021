use std::ops::Index;
use std::str::FromStr;
use std::vec;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bits(Vec<bool>);

impl Bits {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_u32(&self) -> u32 {
        self.0.iter().fold(0, |acc, &bit| acc << 1 | bit as u32)
    }
}

impl Index<usize> for Bits {
    type Output = bool;

    fn index(&self, index: usize) -> &bool {
        &self.0[index]
    }
}

impl IntoIterator for Bits {
    type Item = bool;
    type IntoIter = vec::IntoIter<bool>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromStr for Bits {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let bits = input.chars().map(|c| c == '1').collect();
        Ok(Self(bits))
    }
}
