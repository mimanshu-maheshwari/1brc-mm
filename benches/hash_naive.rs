use criterion::{black_box, criterion_group, criterion_main, Criterion};
use obrc_mm::hash_naive::solve;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hash naive algo", |b| {
        b.iter(|| solve(black_box("res/measurements.txt")))
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);