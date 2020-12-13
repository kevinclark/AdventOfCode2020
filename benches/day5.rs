use aoc2020::day5;

use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn part_1(input: &str) -> usize {
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| day5::seat_id(s.as_bytes()))
        .max()
        .unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = &fs::read_to_string("input/5.txt").unwrap();

    c.bench_function("day5 part 1", |b| b.iter(|| part_1(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
