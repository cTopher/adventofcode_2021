use adventofcode_2021::{day06, parse_file};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day_6(c: &mut Criterion) {
    let input: day06::School = parse_file("src/day06/input.txt");
    c.bench_function("day 6", |b| {
        b.iter(|| day06::part_2(black_box(input.clone())));
    });
}

criterion_group!(benches, day_6);
criterion_main!(benches);
