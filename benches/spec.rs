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

fn bench_presets(c: &mut Criterion) {
    use portspec::{top_100_tcp, top_1000_tcp};
    c.bench_function("top_100_tcp parse", |b| b.iter(top_100_tcp));
    c.bench_function("top_1000_tcp parse", |b| b.iter(top_1000_tcp));
    let small = top_100_tcp();
    c.bench_function("contains in top-100", |b| {
        b.iter(|| black_box(&small).contains(black_box(443)))
    });
}

fn bench_tagged(c: &mut Criterion) {
    use portspec::TaggedSpec;
    let spec = "T:22,80,443,U:53,67-68,123,514";
    c.bench_function("parse tagged spec", |b| {
        b.iter(|| black_box(spec).parse::<TaggedSpec>().unwrap())
    });
}

fn bench_nth_port(c: &mut Criterion) {
    let spec: PortSpec = "1-1024,8000-9000,30000-40000".parse().unwrap();
    c.bench_function("nth_port middle", |b| {
        b.iter(|| black_box(&spec).nth_port(black_box(5000)))
    });
}

criterion_group!(
    benches,
    bench_parse,
    bench_contains,
    bench_set_ops,
    bench_presets,
    bench_tagged,
    bench_nth_port,
);
criterion_main!(benches);
