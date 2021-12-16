use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bits {
    data: Vec<u8>,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
struct Reader<'a> {
    buf: &'a [u8],
}

impl<'a> Reader<'a> {
    fn new(bits: &'a Bits) -> Self {
        Self { buf: &bits.data }
    }

    fn u8(self, n: usize) -> (Self, u8) {
        let mut reader = self;
        let mut value = 0;
        for _ in 0..n {
            value <<= 1;
            let (new, bit) = reader.bit();
            reader = new;
            value |= bit;
        }
        (reader, value)
    }

    fn usize(mut self, mut n: usize) -> (Self, usize) {
        let mut value = 0;
        while n > 8 {
            let (new, byte) = self.u8(8);
            value <<= 8;
            value |= byte as usize;
            n -= 8;
            self = new;
        }
        let (reader, byte) = self.u8(n);
        value <<= n;
        value |= byte as usize;
        (reader, value)
    }

    fn bit(mut self) -> (Self, u8) {
        let val = self.buf[0];
        self.buf = &self.buf[1..];
        (self, val)
    }

    fn bool(self) -> (Self, bool) {
        let (reader, val) = self.bit();
        (reader, val != 0)
    }

    fn take(self, n: usize) -> (Self, Self) {
        (
            Self {
                buf: &self.buf[n..],
            },
            Self {
                buf: &self.buf[..n],
            },
        )
    }

    fn len(self) -> usize {
        self.buf.len()
    }

    fn is_empty(self) -> bool {
        self.buf.is_empty()
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
    fn parse(reader: Reader) -> (Reader, Packet) {
        let (reader, version) = reader.u8(3);
        let (reader, type_id) = reader.u8(3);
        match type_id {
            4 => {
                let (reader, literal) = Literal::parse(version, reader);
                (reader, Packet::Literal(literal))
            }
            _ => {
                let (reader, operator) = Operator::parse(version, type_id, reader);
                (reader, Packet::Operator(operator))
            }
        }
    }

    fn version_sum(&self) -> usize {
        match self {
            Packet::Literal(literal) => literal.version as usize,
            Packet::Operator(operator) => operator.version_sum(),
        }
    }

    fn eval(&self) -> usize {
        match self {
            Packet::Literal(literal) => literal.value as usize,
            Packet::Operator(operator) => operator.eval(),
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
struct Literal {
    version: u8,
    value: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Operator {
    version: u8,
    packets: Vec<Packet>,
    type_id: u8,
}

impl Literal {
    fn parse(version: u8, mut reader: Reader) -> (Reader, Literal) {
        let mut value = 0;
        loop {
            let (inner, prefix) = reader.bool();
            let (inner, v) = inner.u8(4);
            value <<= 4;
            value |= v as u32;
            if !prefix {
                return (inner, Literal { version, value });
            }
            reader = inner
        }
    }
}

impl Operator {
    fn parse(version: u8, type_id: u8, mut reader: Reader) -> (Reader, Operator) {
        let (reader, i) = reader.bit();
        let mut packets = Vec::new();
        let reader = match i {
            0 => {
                let (reader, length) = reader.usize(15);
                let (reader, mut packet_reader) = reader.take(length);
                while packet_reader.len() >= 8 {
                    let (new_reader, packet) = Packet::parse(packet_reader);
                    packet_reader = new_reader;
                    packets.push(packet);
                }
                reader
            }
            1 => {
                let (mut reader, size) = reader.usize(11);
                for _ in 0..size {
                    let (new_reader, packet) = Packet::parse(reader);
                    reader = new_reader;
                    packets.push(packet);
                }
                reader
            }
            _ => unreachable!(),
        };
        (
            reader,
            Operator {
                version,
                packets,
                type_id,
            },
        )
    }

    fn version_sum(&self) -> usize {
        self.version as usize + self.packets.iter().map(|p| p.version_sum()).sum::<usize>()
    }

    fn eval(&self) -> usize {
        let mut vals = self.packets.iter().map(|p| p.eval());
        match self.type_id {
            0 => vals.sum(),
            1 => vals.product(),
            2 => vals.min().unwrap(),
            3 => vals.max().unwrap(),
            5 => (vals.next().unwrap() > vals.next().unwrap()) as usize,
            6 => (vals.next().unwrap() < vals.next().unwrap()) as usize,
            7 => (vals.next().unwrap() == vals.next().unwrap()) as usize,
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
            .flat_map(|b| (0..8).rev().map(move |i| (b >> i) & 1))
            .collect();
        Ok(Self { data })
    }
}

pub fn part_1(bits: Bits) -> usize {
    let reader = Reader::new(&bits);
    let (_, packet) = Packet::parse(reader);
    packet.version_sum()
}

pub fn part_2(bits: Bits) -> usize {
    let reader = Reader::new(&bits);
    let (reader, packet) = Packet::parse(reader);
    println!("{}", reader.len());
    packet.eval()
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    use super::*;


    #[test]
    fn part_1_works() {
       let xxx = |input: &str| {
           let bits = input.parse().unwrap();
           let reader = Reader::new(&bits);
           let (_, packet) = Packet::parse(reader);
           println!("{:?}", packet);
       };

        xxx("8A004A801A8002F478");
        xxx("620080001611562C8802118E34");
        xxx("C0015000016115A2E0802F182340");
        xxx("A0016C880162017C3686B18A3D4780");




        let input: Bits = parse_file("src/day16/input.txt");
        assert_eq!(843, part_1(input));
    }



    #[test]
    fn part_2_tests() {
        assert_eq!(3, part_2("C200B40A82".parse().unwrap()));
        assert_eq!(54, part_2("04005AC33890".parse().unwrap()));
        assert_eq!(7, part_2("880086C3E88112".parse().unwrap()));
        assert_eq!(9, part_2("CE00C43D881120".parse().unwrap()));
        assert_eq!(1, part_2("D8005AC2A8F0".parse().unwrap()));
        assert_eq!(0, part_2("F600BC2D8F".parse().unwrap()));
        assert_eq!(0, part_2("9C005AC2F8F0".parse().unwrap()));
        assert_eq!(1, part_2("9C0141080250320F1802104A08".parse().unwrap()));
    }

    #[test]
    fn part_2_works() {
        // 17803853055 too low
        let input: Bits = parse_file("src/day16/input.txt");
        assert_eq!(0, part_2(input));
    }
}
