use aoc2020::day3;

use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    let input = &fs::read("input/3.txt").unwrap();

    c.bench_function("day3 part 1", |b| b.iter(|| day3::part_1(&input)));

    c.bench_function("day3 part 2", |b| b.iter(|| day3::part_2(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
