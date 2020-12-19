use criterion::{criterion_group, criterion_main, Criterion};

use aoc_2020::problems::problem_18::evaluate_all_expressions;

pub fn part_18_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../resources/problem_18_input.txt");
    c.bench_function("Problem 18 Part 1", |b| b.iter(|| evaluate_all_expressions(input)));
}

criterion_group!(benches, part_18_benchmark);
criterion_main!(benches);