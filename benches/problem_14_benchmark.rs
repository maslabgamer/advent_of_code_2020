use criterion::{criterion_group, criterion_main, Criterion};

use aoc_2020::problems::problem_14::initialize_memory;

pub fn part_14_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../resources/problem_14_input.txt");
    c.bench_function("Problem 14 Part 1", |b| b.iter(|| initialize_memory(input)));
}

criterion_group!(benches, part_14_benchmark);
criterion_main!(benches);