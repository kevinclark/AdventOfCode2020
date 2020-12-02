use aoc2020::{day1, day2};

use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    let input = &fs::read_to_string("input/1.txt")
        .unwrap()
        .trim()
        .split('\n')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    c.bench_function("day1 part 1", |b| b.iter(|| day1::two_sum(2020, &input)));
    c.bench_function("day1 part 2", |b| {
        b.iter(|| day1::three_sum(2020, &input))
    });

    let input = &fs::read_to_string("input/2.txt").unwrap();
    c.bench_function("day2 part 1", |b| {
        b.iter(|| day2::number_of_valid_passwords(input))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
