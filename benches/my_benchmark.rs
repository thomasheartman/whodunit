use criterion::{criterion_group, criterion_main, Criterion};
use whodunit::algorithm;

pub fn bench_algorithm(c: &mut Criterion) {
    let input = std::fs::read_to_string("./input.json").unwrap();

    c.bench_function("algo input", |b| b.iter(|| algorithm(&input)));
}

criterion_group!(benches, bench_algorithm);
criterion_main!(benches);
