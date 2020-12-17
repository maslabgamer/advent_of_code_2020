use criterion::{criterion_group, criterion_main, Criterion};

use aoc_2020::problems::problem_17::run_six_cycles;

pub fn part_16_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../resources/problem_17_input.txt");
    c.bench_function("Problem 17 Part 1", |b| b.iter(|| run_six_cycles(input, false)));
    c.bench_function("Problem 17 Part 2", |b| b.iter(|| run_six_cycles(input, true)));
}

criterion_group!(benches, part_16_benchmark);
criterion_main!(benches);