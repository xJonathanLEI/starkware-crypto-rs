use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hex_literal::hex;
use starkware_crypto_sys::hash;

// Benchmark taken from pathfinder for performance comparison:
// https://github.com/eqlabs/pathfinder/blob/74a173c3dfd66dd92d4980ad3097f153f1951290/crates/pedersen/benches/pedersen.rs

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut e0 = hex!("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb");
    let mut e1 = hex!("0208a0a10250e382e1e4bbe2880906c2791bf6275695e02fbbc6aeff9cd8b31a");

    e0.reverse();
    e1.reverse();

    c.bench_function("pedersen_hash", |b| {
        b.iter(|| {
            black_box(hash(&e0, &e1)).unwrap();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
