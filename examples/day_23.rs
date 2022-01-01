use adventofcode_2021::day23::part_1;
use adventofcode_2021::parse_file;

fn main() {
    let burrow = parse_file("src/day23/input.txt");

    println!("BOEM: {}", part_1(burrow));
}
