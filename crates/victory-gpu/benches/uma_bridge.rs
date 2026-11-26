//! Criterion micro-benchmarks for Subsystem A (`uma_bridge`).
//! Owned by `@uma-bridge`.

use criterion::{Criterion, criterion_group, criterion_main};

fn placeholder(_c: &mut Criterion) {}

criterion_group!(benches, placeholder);
criterion_main!(benches);
