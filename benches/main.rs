use adventofcode_2021::{day06, day11, day12, parse_file};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day_6(c: &mut Criterion) {
    let input: day06::School = parse_file("src/day06/input.txt");
    c.bench_function("day 6", |b| {
        b.iter(|| day06::part_2(black_box(input.clone())));
    });
}

fn day_11(c: &mut Criterion) {
    let input: day11::Cavern = parse_file("src/day11/input.txt");
    c.bench_function("day 11", |b| {
        b.iter(|| day11::part_2(black_box(input.clone())));
    });
}

fn day_12(c: &mut Criterion) {
    let input: day12::CaveSystem = parse_file("src/day12/input.txt");
    c.bench_function("day 12", |b| {
        b.iter(|| day12::part_2(black_box(&input)));
    });
}

criterion_group!(benches, day_6, day_11, day_12);
criterion_main!(benches);
