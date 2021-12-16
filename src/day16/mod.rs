use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bits {
    data: Vec<bool>,
}

macro_rules! parse {
    ($uint:ident) => {
        fn $uint(&mut self, n: usize) -> $uint {
            let mut value = 0;
            for index in 0..n {
                value <<= 1;
                value |= self.buf[index] as $uint;
            }
            self.buf = &self.buf[n..];
            value
        }
    };
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
struct Reader<'a> {
    buf: &'a [bool],
}

impl<'a> Reader<'a> {
    fn new(bits: &'a Bits) -> Self {
        Self { buf: &bits.data }
    }

    parse!(u8);
    parse!(u64);
    parse!(usize);

    fn bit(&mut self) -> u8 {
        self.bool() as u8
    }

    fn bool(&mut self) -> bool {
        let value = self.buf[0];
        self.buf = &self.buf[1..];
        value
    }

    fn take(&mut self, n: usize) -> Self {
        let result = Self {
            buf: &self.buf[..n],
        };
        self.buf = &self.buf[n..];
        result
    }

    const fn len(self) -> usize {
        self.buf.len()
    }
}

impl<'a> fmt::Display for Reader<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for bit in self.buf {
            write!(f, "{}", bit)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    Literal(Literal),
    Operator(Operator),
}

impl Packet {
    fn parse(reader: &mut Reader) -> Self {
        let version = reader.u8(3);
        let type_id = reader.u8(3);
        match type_id {
            4 => Self::Literal(Literal::parse(version, reader)),
            _ => Self::Operator(Operator::parse(version, type_id, reader)),
        }
    }

    fn version_sum(&self) -> u32 {
        match self {
            Self::Literal(literal) => literal.version.into(),
            Self::Operator(operator) => operator.version_sum(),
        }
    }

    fn eval(&self) -> u64 {
        match self {
            Self::Literal(literal) => literal.value as u64,
            Self::Operator(operator) => operator.eval(),
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
struct Literal {
    version: u8,
    value: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Operator {
    version: u8,
    packets: Vec<Packet>,
    type_id: u8,
}

impl Literal {
    fn parse(version: u8, reader: &mut Reader) -> Self {
        let mut value = 0;
        loop {
            let prefix = reader.bool();
            let v = reader.u64(4);
            value <<= 4;
            value |= v;
            if !prefix {
                return Self { version, value };
            }
        }
    }
}

impl Operator {
    fn parse(version: u8, type_id: u8, reader: &mut Reader) -> Self {
        let i = reader.bit();
        let mut packets = Vec::new();
        match i {
            0 => {
                let length = reader.usize(15);
                let mut packet_reader = reader.take(length);
                while packet_reader.len() >= 8 {
                    let packet = Packet::parse(&mut packet_reader);
                    packets.push(packet);
                }
            }
            1 => {
                let size = reader.usize(11);
                for _ in 0..size {
                    let packet = Packet::parse(reader);
                    packets.push(packet);
                }
            }
            _ => unreachable!(),
        };
        Self {
            version,
            packets,
            type_id,
        }
    }

    fn version_sum(&self) -> u32 {
        let packet_sum: u32 = self.packets.iter().map(Packet::version_sum).sum();
        u32::from(self.version) + packet_sum
    }

    fn eval(&self) -> u64 {
        let mut vals = self.packets.iter().map(Packet::eval);
        match self.type_id {
            0 => vals.sum(),
            1 => vals.product(),
            2 => vals.min().unwrap(),
            3 => vals.max().unwrap(),
            5 => (vals.next().unwrap() > vals.next().unwrap()) as u64,
            6 => (vals.next().unwrap() < vals.next().unwrap()) as u64,
            7 => (vals.next().unwrap() == vals.next().unwrap()) as u64,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Bits {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data = (0..input.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
            .flat_map(|b| (0..8).rev().map(move |i| (b >> i) & 1 == 1))
            .collect();
        Ok(Self { data })
    }
}

pub fn part_1(bits: &Bits) -> u32 {
    let mut reader = Reader::new(bits);
    let packet = Packet::parse(&mut reader);
    packet.version_sum()
}

pub fn part_2(bits: &Bits) -> u64 {
    let mut reader = Reader::new(bits);
    let packet = Packet::parse(&mut reader);
    packet.eval()
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    use super::*;

    #[test]
    fn part_1_works() {
        let input: Bits = parse_file("src/day16/input.txt");
        assert_eq!(843, part_1(&input));
    }

    #[test]
    fn part_2_tests() {
        assert_eq!(3, part_2(&"C200B40A82".parse().unwrap()));
        assert_eq!(54, part_2(&"04005AC33890".parse().unwrap()));
        assert_eq!(7, part_2(&"880086C3E88112".parse().unwrap()));
        assert_eq!(9, part_2(&"CE00C43D881120".parse().unwrap()));
        assert_eq!(1, part_2(&"D8005AC2A8F0".parse().unwrap()));
        assert_eq!(0, part_2(&"F600BC2D8F".parse().unwrap()));
        assert_eq!(0, part_2(&"9C005AC2F8F0".parse().unwrap()));
        assert_eq!(1, part_2(&"9C0141080250320F1802104A08".parse().unwrap()));
    }

    #[test]
    fn part_2_works() {
        let input: Bits = parse_file("src/day16/input.txt");
        assert_eq!(5_390_807_940_351, part_2(&input));
    }
}
