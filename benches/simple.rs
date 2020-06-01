use criterion::{criterion_group, criterion_main, Criterion};
use simple::run;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("run", |b| b.iter(|| run()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);