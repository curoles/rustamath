use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustamath::foo;

pub fn foo_benchmark(c: &mut Criterion) {
    c.bench_function("foo 20", |b| b.iter(|| foo(black_box(20))));
}

// https://docs.rs/criterion/latest/criterion/struct.BenchmarkGroup.html

criterion_group!(benches, foo_benchmark);
criterion_main!(benches);
