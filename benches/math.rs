use criterion::{black_box, criterion_group, criterion_main, Criterion};
use toolchest::math::*;

fn bench_rounding(c: &mut Criterion) {
    c.bench_function("round_2dp", |b| b.iter(|| round(black_box(3.14159), 2)));
    c.bench_function("ceil_2dp", |b| b.iter(|| ceil(black_box(3.14159), 2)));
    c.bench_function("floor_2dp", |b| b.iter(|| floor(black_box(3.14159), 2)));
}

fn bench_clamp(c: &mut Criterion) {
    c.bench_function("clamp_i32", |b| b.iter(|| clamp(black_box(15), 0, 10)));
}

criterion_group!(benches, bench_rounding, bench_clamp);
criterion_main!(benches);


