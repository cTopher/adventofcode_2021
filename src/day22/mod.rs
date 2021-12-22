use std::cmp;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub struct Cuboid {
    min: Vector,
    max: Vector,
    on: bool,
}

impl Cuboid {
    fn compensate(&self, other: &Self) -> Option<Self> {
        let min = Vector {
            x: cmp::max(self.min.x, other.min.x),
            y: cmp::max(self.min.y, other.min.y),
            z: cmp::max(self.min.z, other.min.z),
        };
        let max = Vector {
            x: cmp::min(self.max.x, other.max.x),
            y: cmp::min(self.max.y, other.max.y),
            z: cmp::min(self.max.z, other.max.z),
        };
        if min.x > max.x || min.y > max.y || min.z > max.z {
            None
        } else {
            Some(Self {
                min,
                max,
                on: !self.on,
            })
        }
    }

    #[allow(clippy::cast_sign_loss)]
    const fn volume(&self) -> u64 {
        let x = (self.max.x - self.min.x + 1) as u64;
        let y = (self.max.y - self.min.y + 1) as u64;
        let z = (self.max.z - self.min.z + 1) as u64;
        x * y * z
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct ReactorCore {
    cubes: Vec<Cuboid>,
}

impl ReactorCore {
    fn add(&mut self, cuboid: Cuboid) {
        let mut counter_cubes = self
            .cubes
            .iter()
            .filter_map(|c| c.compensate(&cuboid))
            .collect();
        self.cubes.append(&mut counter_cubes);
        if cuboid.on {
            self.cubes.push(cuboid);
        }
    }

    fn count(&self) -> u64 {
        self.cubes.iter().fold(0, |acc, cuboid| {
            if cuboid.on {
                acc + cuboid.volume()
            } else {
                acc - cuboid.volume()
            }
        })
    }
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let on = input.starts_with("on");
        let coords: Vec<i32> = input
            .split_once(' ')
            .unwrap()
            .1
            .split(',')
            .flat_map(|s| s[2..].split(".."))
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        Ok(Self {
            min: Vector {
                x: coords[0],
                y: coords[2],
                z: coords[4],
            },
            max: Vector {
                x: coords[1],
                y: coords[3],
                z: coords[5],
            },
            on,
        })
    }
}

pub fn part_1(cuboids: impl Iterator<Item = Cuboid>) -> u64 {
    let mut core = ReactorCore::default();
    for cuboid in cuboids {
        let Cuboid { min, max, .. } = &cuboid;
        if min.x >= -50 && max.x <= 50 && min.y >= -50 && max.y <= 50 && min.z >= -50 && max.z <= 50
        {
            core.add(cuboid);
        }
    }
    core.count()
}

pub fn part_2(cuboids: impl Iterator<Item = Cuboid>) -> u64 {
    let mut core = ReactorCore::default();
    for cuboid in cuboids {
        core.add(cuboid);
    }
    core.count()
}

#[cfg(test)]
mod tests {
    use crate::{parse_file_lines, parse_str_lines};

    use super::*;

    #[test]
    fn example_1_produces_590784() {
        let cuboids = parse_str_lines(EXAMPLE_1);
        assert_eq!(590_784, part_1(cuboids));
    }

    #[test]
    fn part_1_works() {
        let mut cuboids = parse_file_lines("src/day22/input.txt");
        assert_eq!(596_598, part_1(&mut cuboids));
    }

    #[test]
    fn example_2_produces_2758514936282235() {
        let cuboids = parse_str_lines(EXAMPLE_2);
        assert_eq!(2_758_514_936_282_235, part_2(cuboids));
    }

    #[test]
    fn part_2_works() {
        let mut cuboids = parse_file_lines("src/day22/input.txt");
        assert_eq!(1_199_121_349_148_621, part_2(&mut cuboids));
    }

    const EXAMPLE_1: &str = "\
        on x=-20..26,y=-36..17,z=-47..7\n\
        on x=-20..33,y=-21..23,z=-26..28\n\
        on x=-22..28,y=-29..23,z=-38..16\n\
        on x=-46..7,y=-6..46,z=-50..-1\n\
        on x=-49..1,y=-3..46,z=-24..28\n\
        on x=2..47,y=-22..22,z=-23..27\n\
        on x=-27..23,y=-28..26,z=-21..29\n\
        on x=-39..5,y=-6..47,z=-3..44\n\
        on x=-30..21,y=-8..43,z=-13..34\n\
        on x=-22..26,y=-27..20,z=-29..19\n\
        off x=-48..-32,y=26..41,z=-47..-37\n\
        on x=-12..35,y=6..50,z=-50..-2\n\
        off x=-48..-32,y=-32..-16,z=-15..-5\n\
        on x=-18..26,y=-33..15,z=-7..46\n\
        off x=-40..-22,y=-38..-28,z=23..41\n\
        on x=-16..35,y=-41..10,z=-47..6\n\
        off x=-32..-23,y=11..30,z=-14..3\n\
        on x=-49..-5,y=-3..45,z=-29..18\n\
        off x=18..30,y=-20..-8,z=-3..13\n\
        on x=-41..9,y=-7..43,z=-33..15\n\
        on x=-54112..-39298,y=-85059..-49293,z=-27449..7877\n\
        on x=967..23432,y=45373..81175,z=27513..53682\
    ";

    const EXAMPLE_2: &str = "\
        on x=-5..47,y=-31..22,z=-19..33\n\
        on x=-44..5,y=-27..21,z=-14..35\n\
        on x=-49..-1,y=-11..42,z=-10..38\n\
        on x=-20..34,y=-40..6,z=-44..1\n\
        off x=26..39,y=40..50,z=-2..11\n\
        on x=-41..5,y=-41..6,z=-36..8\n\
        off x=-43..-33,y=-45..-28,z=7..25\n\
        on x=-33..15,y=-32..19,z=-34..11\n\
        off x=35..47,y=-46..-34,z=-11..5\n\
        on x=-14..36,y=-6..44,z=-16..29\n\
        on x=-57795..-6158,y=29564..72030,z=20435..90618\n\
        on x=36731..105352,y=-21140..28532,z=16094..90401\n\
        on x=30999..107136,y=-53464..15513,z=8553..71215\n\
        on x=13528..83982,y=-99403..-27377,z=-24141..23996\n\
        on x=-72682..-12347,y=18159..111354,z=7391..80950\n\
        on x=-1060..80757,y=-65301..-20884,z=-103788..-16709\n\
        on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856\n\
        on x=-52752..22273,y=-49450..9096,z=54442..119054\n\
        on x=-29982..40483,y=-108474..-28371,z=-24328..38471\n\
        on x=-4958..62750,y=40422..118853,z=-7672..65583\n\
        on x=55694..108686,y=-43367..46958,z=-26781..48729\n\
        on x=-98497..-18186,y=-63569..3412,z=1232..88485\n\
        on x=-726..56291,y=-62629..13224,z=18033..85226\n\
        on x=-110886..-34664,y=-81338..-8658,z=8914..63723\n\
        on x=-55829..24974,y=-16897..54165,z=-121762..-28058\n\
        on x=-65152..-11147,y=22489..91432,z=-58782..1780\n\
        on x=-120100..-32970,y=-46592..27473,z=-11695..61039\n\
        on x=-18631..37533,y=-124565..-50804,z=-35667..28308\n\
        on x=-57817..18248,y=49321..117703,z=5745..55881\n\
        on x=14781..98692,y=-1341..70827,z=15753..70151\n\
        on x=-34419..55919,y=-19626..40991,z=39015..114138\n\
        on x=-60785..11593,y=-56135..2999,z=-95368..-26915\n\
        on x=-32178..58085,y=17647..101866,z=-91405..-8878\n\
        on x=-53655..12091,y=50097..105568,z=-75335..-4862\n\
        on x=-111166..-40997,y=-71714..2688,z=5609..50954\n\
        on x=-16602..70118,y=-98693..-44401,z=5197..76897\n\
        on x=16383..101554,y=4615..83635,z=-44907..18747\n\
        off x=-95822..-15171,y=-19987..48940,z=10804..104439\n\
        on x=-89813..-14614,y=16069..88491,z=-3297..45228\n\
        on x=41075..99376,y=-20427..49978,z=-52012..13762\n\
        on x=-21330..50085,y=-17944..62733,z=-112280..-30197\n\
        on x=-16478..35915,y=36008..118594,z=-7885..47086\n\
        off x=-98156..-27851,y=-49952..43171,z=-99005..-8456\n\
        off x=2032..69770,y=-71013..4824,z=7471..94418\n\
        on x=43670..120875,y=-42068..12382,z=-24787..38892\n\
        off x=37514..111226,y=-45862..25743,z=-16714..54663\n\
        off x=25699..97951,y=-30668..59918,z=-15349..69697\n\
        off x=-44271..17935,y=-9516..60759,z=49131..112598\n\
        on x=-61695..-5813,y=40978..94975,z=8655..80240\n\
        off x=-101086..-9439,y=-7088..67543,z=33935..83858\n\
        off x=18020..114017,y=-48931..32606,z=21474..89843\n\
        off x=-77139..10506,y=-89994..-18797,z=-80..59318\n\
        off x=8476..79288,y=-75520..11602,z=-96624..-24783\n\
        on x=-47488..-1262,y=24338..100707,z=16292..72967\n\
        off x=-84341..13987,y=2429..92914,z=-90671..-1318\n\
        off x=-37810..49457,y=-71013..-7894,z=-105357..-13188\n\
        off x=-27365..46395,y=31009..98017,z=15428..76570\n\
        off x=-70369..-16548,y=22648..78696,z=-1892..86821\n\
        on x=-53470..21291,y=-120233..-33476,z=-44150..38147\n\
        off x=-93533..-4276,y=-16170..68771,z=-104985..-24507\
    ";
}
