use criterion::{black_box, criterion_group, criterion_main, Criterion};
use toolchest::strings::*;

fn bench_case_conversions(c: &mut Criterion) {
    let input = "HelloWorldThisIsATest";
    
    c.bench_function("to_snake_case", |b| {
        b.iter(|| to_snake_case(black_box(input)))
    });
    
    c.bench_function("to_camel_case", |b| {
        b.iter(|| to_camel_case(black_box(input)))
    });
    
    c.bench_function("to_kebab_case", |b| {
        b.iter(|| to_kebab_case(black_box(input)))
    });
}

fn bench_string_manipulation(c: &mut Criterion) {
    let long_string = "a".repeat(1000);
    
    c.bench_function("truncate_long", |b| {
        b.iter(|| truncate(black_box(&long_string), 50))
    });
    
    c.bench_function("pad_start", |b| {
        b.iter(|| pad_start(black_box("5"), 10, '0'))
    });
}

criterion_group!(benches, bench_case_conversions, bench_string_manipulation);
criterion_main!(benches);


