use aoc2020::day2;

use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    let raw_input = &fs::read("input/2.txt").unwrap();
    let input = &raw_input[..raw_input.len() - 1]; // trim newline

    c.bench_function("day2 part 1", |b| {
        b.iter(|| day2::number_of_valid_part_1_passwords(&input))
    });

    c.bench_function("day2 part 2", |b| {
        b.iter(|| day2::number_of_valid_part_2_passwords(&input))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
