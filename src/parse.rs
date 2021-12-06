use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::{fmt, fs};

pub fn parse_file<E: fmt::Debug, F: FromStr<Err = E>>(filename: &str) -> F {
    fs::read_to_string(filename)
        .unwrap()
        .trim()
        .parse()
        .unwrap()
}

pub fn parse_file_lines<E: fmt::Debug, F: FromStr<Err = E>>(
    filename: &str,
) -> impl Iterator<Item = F> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
}

pub fn parse_str_lines<E: fmt::Debug, F: FromStr<Err = E>>(
    input: &'static str,
) -> impl Iterator<Item = F> {
    input.lines().map(|line| line.parse().unwrap())
}
