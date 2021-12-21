use adventofcode_2021::{
    day06, day11, day12, day15, day17, day18, day19, day20, parse_file, parse_file_lines,
};
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

fn day_15(c: &mut Criterion) {
    let input: day15::CavernInput = parse_file("src/day15/input.txt");
    c.bench_function("day 15", |b| {
        b.iter(|| day15::part_2(black_box(input.clone())));
    });
}

fn day_17(c: &mut Criterion) {
    let input: day17::TargetArea = parse_file("src/day17/input.txt");
    c.bench_function("day 17", |b| {
        b.iter(|| day17::part_2(black_box(input)));
    });
}

fn day_18(c: &mut Criterion) {
    c.bench_function("day 18", |b| {
        b.iter(|| day18::part_2(black_box(parse_file_lines("src/day18/input.txt"))));
    });
}

fn day_19(c: &mut Criterion) {
    let input: day19::BeaconMap = parse_file("src/day19/input.txt");
    c.bench_function("day 19", |b| {
        b.iter(|| day19::part_2(black_box(input.clone())));
    });
}

fn day_20(c: &mut Criterion) {
    let input: day20::Input = parse_file("src/day20/input.txt");
    c.bench_function("day 20", |b| {
        b.iter(|| day20::part_2(black_box(input.clone())));
    });
}

criterion_group!(benches, day_6, day_11, day_12, day_15, day_17, day_18, day_19, day_20);
criterion_main!(benches);
