mod dot;
mod fold;
mod paper;

use dot::Dot;
use fold::Fold;
use paper::Paper;

pub fn part_1(mut paper: Paper) -> usize {
    paper.do_first_instruction();
    paper.number_of_dots()
}

pub fn part_2(mut paper: Paper) -> String {
    paper.do_instructions();
    let paper = paper.to_string();
    println!("{}", paper);
    paper
}

#[cfg(test)]
mod tests {
    use crate::parse_file;

    use super::*;

    const EXAMPLE: &str = "\
        6,10\n\
        0,14\n\
        9,10\n\
        0,3\n\
        10,4\n\
        4,11\n\
        6,0\n\
        6,12\n\
        4,1\n\
        0,13\n\
        10,12\n\
        3,4\n\
        3,0\n\
        8,4\n\
        1,10\n\
        2,14\n\
        8,10\n\
        9,0\n\
        \n\
        fold along y=7\n\
        fold along x=5\
    ";

    #[test]
    fn example_1_produces_17() {
        let paper = EXAMPLE.parse().unwrap();
        assert_eq!(17, part_1(paper));
    }

    #[test]
    fn part_1_works() {
        let paper = parse_file("src/day13/input.txt");
        assert_eq!(701, part_1(paper));
    }

    #[test]
    fn example_2_produces_a_square() {
        let paper = EXAMPLE.parse().unwrap();
        assert_eq!(
            "\
                #####\n\
                #...#\n\
                #...#\n\
                #...#\n\
                #####\n\
                .....\n\
                .....\n\
            ",
            part_2(paper)
        );
    }

    #[test]
    fn part_2_works() {
        let paper = parse_file("src/day13/input.txt");
        assert_eq!(
            "\
                ####.###..####.#..#.###..####...##.#....\n\
                #....#..#.#....#.#..#..#.#.......#.#....\n\
                ###..#..#.###..##...###..###.....#.#....\n\
                #....###..#....#.#..#..#.#.......#.#....\n\
                #....#....#....#.#..#..#.#....#..#.#....\n\
                #....#....####.#..#.###..####..##..####.\n\
            ",
            part_2(paper)
        );
    }
}
