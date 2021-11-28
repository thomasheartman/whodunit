use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

struct Res<'a>(&'a str, &'a str, &'a str);

fn split_collect(s: &str) -> Result<Res, &str> {
    let segments: Vec<&str> = s.split(':').collect();
    match &segments as &[&str] {
        [location, object_id, code, ..] => Ok(Res(location, object_id, code)),
        _ => Err("Couldn't convert string"),
    }
}

fn split_iter_next(s: &str) -> Result<Res, &str> {
    let mut segments = s.split(':');
    let loc = segments.next().unwrap();
    let id = segments.next().unwrap();
    let code = segments.next().unwrap();

    Ok(Res(loc, id, code))
}

fn find_indices_manual_split(s: &str) -> Result<Res, &str> {
    let first = s.find(':').unwrap();
    let last = s.rfind(':').unwrap();

    Ok(Res(&s[..first], &s[first + 1..last], &s[last + 1..]))
}

fn find_indices_manual_split_from_start(s: &str) -> Result<Res, &str> {
    let first = s.find(':').unwrap();
    let last = s[first + 1..].find(':').unwrap();

    Ok(Res(&s[..first], &s[first + 1..last], &s[last + 1..]))
}

fn split_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("Split");

    let input = "insuresys:61a250e1f5025a768a3172ac:aliquip";

    group.bench_with_input(
        BenchmarkId::new("Split collect", input),
        input,
        |b, input| b.iter(|| split_collect(input)),
    );

    group.bench_with_input(
        BenchmarkId::new("Split iter next", input),
        input,
        |b, input| b.iter(|| split_iter_next(input)),
    );

    group.bench_with_input(
        BenchmarkId::new("Manual index", input),
        input,
        |b, input| b.iter(|| find_indices_manual_split(input)),
    );

    group.bench_with_input(
        BenchmarkId::new("Manual index from start", input),
        input,
        |b, input| b.iter(|| find_indices_manual_split_from_start(input)),
    );

    group.finish();
}

criterion_group!(string_benches, split_string);
criterion_main!(string_benches);
