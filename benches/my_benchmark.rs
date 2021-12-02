use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use whodunit::algorithm;

pub fn bench_algorithm_unique_inputs(c: &mut Criterion) {
    let x10 = std::fs::read_to_string("./10.json").unwrap();
    let x100 = std::fs::read_to_string("./100.json").unwrap();
    let x1_000 = std::fs::read_to_string("./1000.json").unwrap();
    let x10_000 = std::fs::read_to_string("./10_000-unique.json").unwrap();

    let mut group = c.benchmark_group("Unique inputs");

    for (i, f) in [x10, x100, x1_000, x10_000].iter().enumerate() {
        let zeroes = "0".repeat(i + 1);
        let id = format!("1{zeroes} unique elements");
        group.bench_with_input(BenchmarkId::from_parameter(id), f, |b, input| {
            b.iter(|| algorithm(input));
        });
    }

    group.finish();
}

pub fn bench_algorithm_with_duplicates(c: &mut Criterion) {
    let x20_000_two_of_each = std::fs::read_to_string("./20_000-two-of-each.json").unwrap();
    let mut group = c.benchmark_group("Algo range with dupes");
    for (f, text) in [(x20_000_two_of_each, "20,000 (10,000 unique)")].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(text), f, |b, input| {
            b.iter(|| algorithm(input));
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_algorithm_unique_inputs,
    bench_algorithm_with_duplicates,
);
criterion_main!(benches);
