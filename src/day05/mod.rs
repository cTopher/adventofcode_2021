pub use line::Line;
pub use point::Point;

mod line;
mod point;

#[derive(Clone, Debug)]
struct Diagram(Vec<Vec<u8>>);

impl Diagram {
    const fn new() -> Self {
        Self(Vec::new())
    }

    fn add_line(&mut self, line: Line) {
        for point in line.points() {
            self.add_point(point);
        }
    }

    fn row_mut(&mut self, row: usize) -> &mut Vec<u8> {
        if self.0.len() <= row {
            self.0.resize(row + 1, Vec::new());
        }
        &mut self.0[row]
    }

    fn add_point(&mut self, point: Point) {
        let row = self.row_mut(point.y);
        if row.len() <= point.x {
            row.resize(point.x + 1, 0);
        }
        row[point.x] += 1;
    }

    fn amount_of_danger_points(&self) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|&&x| x > 1).count())
            .sum()
    }
}

pub fn part_1<I: Iterator<Item = Line>>(lines: I) -> usize {
    let lines = lines
        .into_iter()
        .filter(|line| line.horizontal() || line.vertical());
    part_2(lines)
}

pub fn part_2<I: Iterator<Item = Line>>(lines: I) -> usize {
    let mut diagram = Diagram::new();
    for line in lines {
        diagram.add_line(line);
    }
    diagram.amount_of_danger_points()
}

#[cfg(test)]
mod tests {
    use crate::{parse_file_lines, parse_str_lines};

    use super::*;

    const EXAMPLE: &str = "\
        0,9 -> 5,9\n\
        8,0 -> 0,8\n\
        9,4 -> 3,4\n\
        2,2 -> 2,1\n\
        7,0 -> 7,4\n\
        6,4 -> 2,0\n\
        0,9 -> 2,9\n\
        3,4 -> 1,4\n\
        0,0 -> 8,8\n\
        5,5 -> 8,2\
    ";

    #[test]
    fn example_1_produces_5() {
        let input = parse_str_lines(EXAMPLE);
        assert_eq!(5, part_1(input));
    }

    #[test]
    fn part_1_works() {
        let input = parse_file_lines("src/day05/input.txt");
        assert_eq!(6225, part_1(input));
    }

    #[test]
    fn example_2_produces_12() {
        let input = parse_str_lines(EXAMPLE);
        assert_eq!(12, part_2(input));
    }

    #[test]
    fn part_2_works() {
        let input = parse_file_lines("src/day05/input.txt");
        assert_eq!(22116, part_2(input));
    }
}
