use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn parse_file<E: fmt::Debug, F: FromStr<Err = E>>(filename: &str) -> impl Iterator<Item = F> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
}
