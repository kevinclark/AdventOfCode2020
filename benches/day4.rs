use aoc2020::day4;

use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

fn criterion_benchmark(c: &mut Criterion) {
    let input = &fs::read_to_string("input/4.txt").unwrap();

    c.bench_function("day4 part 1", |b| {
        b.iter(|| day4::number_of_passports_with_valid_fields(&input))
    });

    c.bench_function("day4 part 2", |b| {
        b.iter(|| {
            day4::number_of_passports_with_valid_fields_and_values(&input)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
