use criterion::{criterion_group, criterion_main, Criterion};

use xid;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("xid::new()", |b| b.iter(|| xid::new()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
