use adventofcode_2021::day21::part_2;
use adventofcode_2021::parse_file;

fn main() {
    let game = parse_file("src/day21/input.txt");

    println!("BOEM: {}", part_2(game));
}
