use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rustamath::polynomial::{polynomial_n, naive_polynomial_n};

// https://bheisler.github.io/criterion.rs/book/user_guide/comparing_functions.html
//
fn polynomial_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Polynomial");
    let x: f64 = 123.4567;
    let c: [f64; 1001] = [345.6789; 1001];

    for i in [20, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("Naive", i), i,
            |b, i| b.iter(|| naive_polynomial_n(x, &c[..*i+1])));
        group.bench_with_input(BenchmarkId::new("Horner", i), i,
            |b, i| b.iter(|| polynomial_n(x, &c[..*i+1])));
    }
    group.finish();
}

criterion_group!(benches, polynomial_benchmark);
criterion_main!(benches);
