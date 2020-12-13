use criterion::{criterion_group, criterion_main, Criterion};

use aoc_2020::ship::run_base;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("problem_12", |b| b.iter(|| run_base()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
