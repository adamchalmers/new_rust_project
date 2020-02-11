use criterion::{criterion_group, criterion_main, Criterion};
use newproject::foo;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("main bench", |b| {
        b.iter(|| {
            foo();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
