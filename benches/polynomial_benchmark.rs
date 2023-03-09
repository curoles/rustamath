
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustomath::foo;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("foo 20", |b| b.iter(|| foo(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
