use criterion::{criterion_group, criterion_main, Criterion};

extern crate ignition;
use ignition::prelude::*;

fn initialization(c: &mut Criterion) {
    c.bench_function("Engine initialization", |b| b.iter(|| Engine::ignite()));
}

criterion_group!(init, initialization);
criterion_main!(init);
