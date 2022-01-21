use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hex_literal::hex;
use starkware_crypto_sys::get_public_key;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut private_key = hex!("04a724706e80e5ea88b9ee60a7ede83cbc2de27da0659bef2929381a298b672d");

    private_key.reverse();

    c.bench_function("ecdsa_get_public_key", |b| {
        b.iter(|| {
            black_box(get_public_key(&private_key)).unwrap();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
