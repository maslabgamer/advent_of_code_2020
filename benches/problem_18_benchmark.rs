use criterion::{criterion_group, criterion_main, Criterion};

use aoc_2020::problems::problem_18::{evaluate_all_expressions, swapped_precedence};

pub fn part_18_benchmark(c: &mut Criterion) {
    let input = include_bytes!("../resources/problem_18_input.txt");
    c.bench_function("Problem 18 Part 1", |b| b.iter(|| evaluate_all_expressions(input)));
    c.bench_function("Problem 18 Part 2", |b| b.iter(|| swapped_precedence(input)));
}

criterion_group!(benches, part_18_benchmark);
criterion_main!(benches);