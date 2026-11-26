//! Criterion micro-benchmarks for Subsystem B (`velocity_io`).
//! Owned by `@velocity-io`.

use criterion::{Criterion, criterion_group, criterion_main};

fn placeholder(_c: &mut Criterion) {}

criterion_group!(benches, placeholder);
criterion_main!(benches);
