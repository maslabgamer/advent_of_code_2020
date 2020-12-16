use criterion::{criterion_group, criterion_main, Criterion};

use aoc_2020::problems::problem_16::{find_invalid_values, parse_valid_tickets};

pub fn part_16_benchmark(c: &mut Criterion) {
    let input = include_str!("../resources/problem_16_input.txt");
    c.bench_function("Problem 16 Part 1", |b| b.iter(|| find_invalid_values(input)));
    c.bench_function("Problem 16 Part 2", |b| b.iter(|| parse_valid_tickets(input)));
}

criterion_group!(benches, part_16_benchmark);
criterion_main!(benches);
