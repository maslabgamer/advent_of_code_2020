use criterion::{criterion_group, criterion_main, Criterion};

use aoc_2020::problems::problem_13::{find_earliest_bus, find_sequential_arrivals};

pub fn part_13_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../resources/problem_13_input.txt");
    c.bench_function("Part 1", |b| b.iter(|| find_earliest_bus(input)));
    c.bench_function("Part 2", |b| b.iter(|| find_sequential_arrivals(input)));
}

criterion_group!(benches, part_13_benchmark);
criterion_main!(benches);
