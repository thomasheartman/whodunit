use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use whodunit::{fibonacci, fibonacci_fast};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibs");

    for i in [20u64, 21u64, 22u64].iter() {
        group.bench_with_input(BenchmarkId::new("Recursive", i), i, |b, i| {
            b.iter(|| fibonacci(*i))
        });
        group.bench_with_input(BenchmarkId::new("Iterative", i), i, |b, i| {
            b.iter(|| fibonacci_fast(*i))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
