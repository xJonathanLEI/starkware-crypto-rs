use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hex_literal::hex;
use starkware_crypto_sys::sign;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut private_key = hex!("04a724706e80e5ea88b9ee60a7ede83cbc2de27da0659bef2929381a298b672d");
    let mut message = hex!("010aaf60f545a5b9a55463fbb56f35dfdfe8010ff1d95283afe1b14e07cb8f61");
    let mut k = hex!("075414c392c57a61417fc1702ad6fa83d12541690963915646617b59451972b3");

    private_key.reverse();
    message.reverse();
    k.reverse();

    c.bench_function("ecdsa_sign", |b| {
        b.iter(|| {
            black_box(sign(&private_key, &message, &k).unwrap());
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
