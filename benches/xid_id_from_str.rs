use std::str::FromStr;

use criterion::{criterion_group, criterion_main, Criterion};

use xid::Id;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("xid::Id::from_str()", |b| {
        b.iter(|| Id::from_str("9m4e2mr0ui3e8a215n4g"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
