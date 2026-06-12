//! Microbenchmarks for parsing, containment, and set operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use portspec::PortSpec;

fn bench_parse(c: &mut Criterion) {
    let input = "1-1024,3306,5432,8000-8100,9200,443,22,80";
    c.bench_function("parse spec", |b| {
        b.iter(|| black_box(input).parse::<PortSpec>().unwrap())
    });
}

fn bench_contains(c: &mut Criterion) {
    let spec: PortSpec = "1-1024,8000-9000,30000-40000".parse().unwrap();
    c.bench_function("contains", |b| {
        b.iter(|| black_box(&spec).contains(black_box(8443)))
    });
}

fn bench_set_ops(c: &mut Criterion) {
    let a: PortSpec = "1-20000".parse().unwrap();
    let b: PortSpec = "10000-30000".parse().unwrap();
    c.bench_function("union", |bn| bn.iter(|| black_box(&a).union(black_box(&b))));
    c.bench_function("intersection", |bn| {
        bn.iter(|| black_box(&a).intersection(black_box(&b)))
    });
    c.bench_function("difference", |bn| {
        bn.iter(|| black_box(&a).difference(black_box(&b)))
    });
}

criterion_group!(benches, bench_parse, bench_contains, bench_set_ops);
criterion_main!(benches);
