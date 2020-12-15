use criterion::{criterion_group, criterion_main, Criterion};

use aoc_2020::ship::run_base;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Problem 12 Both Parts", |b| b.iter(|| run_base()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
