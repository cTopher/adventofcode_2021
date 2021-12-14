enum CharStatus {
    Open,
    Close,
    Illegal(u64),
}

enum LineStatus {
    Corrupt(u64),
    Incomplete(Vec<char>),
}

impl CharStatus {
    fn new(last_open: Option<char>, new: char) -> Self {
        use CharStatus::{Close, Illegal, Open};
        match (last_open, new) {
            (_, '(' | '[' | '{' | '<') => Open,
            (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => Close,
            (_, ')') => Illegal(3),
            (_, ']') => Illegal(57),
            (_, '}') => Illegal(1197),
            (_, '>') => Illegal(25137),
            _ => panic!("WTF mate: '{:?}' '{}'", last_open, new),
        }
    }
}

fn process(line: &str) -> LineStatus {
    let mut open: Vec<char> = Vec::new();
    for c in line.chars() {
        match CharStatus::new(open.last().copied(), c) {
            CharStatus::Open => open.push(c),
            CharStatus::Close => {
                open.pop().unwrap();
            }
            CharStatus::Illegal(score) => return LineStatus::Corrupt(score),
        }
    }
    LineStatus::Incomplete(open)
}

fn syntax_error_score(line: &str) -> u64 {
    match process(line) {
        LineStatus::Corrupt(score) => score,
        LineStatus::Incomplete(_) => 0,
    }
}

fn autocomplete_score(line: &str) -> Option<u64> {
    let score_fn = |char| match char {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("WTF mate: '{}'", char),
    };
    if let LineStatus::Incomplete(open) = process(line) {
        let score = open
            .into_iter()
            .rev()
            .map(score_fn)
            .reduce(|a, b| a * 5 + b)
            .unwrap();
        Some(score)
    } else {
        None
    }
}

pub fn part_1<I: Iterator<Item = String>>(lines: I) -> u64 {
    lines.map(|line| syntax_error_score(&line)).sum()
}

pub fn part_2<I: Iterator<Item = String>>(lines: I) -> u64 {
    let mut scores: Vec<u64> = lines.filter_map(|line| autocomplete_score(&line)).collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use crate::{parse_file_lines, parse_str_lines};

    use super::*;

    const EXAMPLE: &str = "\
        [({(<(())[]>[[{[]{<()<>>\n\
        [(()[<>])]({[<{<<[]>>(\n\
        {([(<{}[<>[]}>{[]{[(<()>\n\
        (((({<>}<{<{<>}{[]{[]{}\n\
        [[<[([]))<([[{}[[()]]]\n\
        [{[{({}]{}}([{[{{{}}([]\n\
        {<[[]]>}<{[{[{[]{()[[[]\n\
        [<(<(<(<{}))><([]([]()\n\
        <{([([[(<>()){}]>(<<{{\n\
        <{([{{}}[<[[[<>{}]]]>[]]\
    ";

    #[test]
    fn example_1_produces_26397() {
        let lines = parse_str_lines(EXAMPLE);
        assert_eq!(26397, part_1(lines));
    }

    #[test]
    fn part_1_works() {
        let lines = parse_file_lines("src/day10/input.txt");
        assert_eq!(323_691, part_1(lines));
    }

    #[test]
    fn example_2_produces_288957() {
        let lines = parse_str_lines(EXAMPLE);
        assert_eq!(288_957, part_2(lines));
    }

    #[test]
    fn part_2_works() {
        let lines = parse_file_lines("src/day10/input.txt");
        assert_eq!(2_858_785_164, part_2(lines));
    }
}
