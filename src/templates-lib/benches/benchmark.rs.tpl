use criterion::{black_box, criterion_group, criterion_main, Criterion};

use {name}::add;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench", |b| {
        b.iter(|| {
           add(1, 2);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
