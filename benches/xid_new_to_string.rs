use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("xid::new().to_string()", |b| {
        b.iter(|| xid::new().to_string())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
