use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use whodunit::algorithm;

pub fn bench_algorithm(c: &mut Criterion) {
    let x10 = std::fs::read_to_string("./10.json").unwrap();
    let x100 = std::fs::read_to_string("./100.json").unwrap();
    let x1_000 = std::fs::read_to_string("./1000.json").unwrap();
    let x10_000 = std::fs::read_to_string("./10_000.json").unwrap();
    let x100_000 = std::fs::read_to_string("./100_000.json").unwrap();

    let mut group = c.benchmark_group("Algo range");

    for (i, f) in [x10, x100, x1_000, x10_000, x100_000].iter().enumerate() {
        let zeroes = "0".repeat(i + 1);
        let id = format!("1{zeroes} elements");
        group.bench_with_input(BenchmarkId::from_parameter(id), f, |b, input| {
            b.iter(|| algorithm(input));
        });
    }

    group.finish();
}

pub fn bench_algorithm_with_duplicates(c: &mut Criterion) {
    let x200_000 = std::fs::read_to_string("./200_000.json").unwrap();

    let mut group = c.benchmark_group("Algo range with dupes");
    for (i, f) in [x200_000].iter().enumerate() {
        let zeroes = "0".repeat(i + 5);
        let id = format!("2{zeroes} elements (duplicates >= 50%)");
        // let duplicated = f.repeat(2);
        group.bench_with_input(BenchmarkId::from_parameter(id), f, |b, input| {
            b.iter(|| algorithm(input));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_algorithm, bench_algorithm_with_duplicates);
criterion_main!(benches);
