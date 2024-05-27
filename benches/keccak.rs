use criterion::{criterion_group, criterion_main, Criterion};

use mirith::fips202::sha3_512;

fn bench_sha3_512(c: &mut Criterion) {
    c.bench_function("bench_sha3_512", |b| {
        b.iter(|| {
            std::hint::black_box(for _i in 1..=5 {
                let in0 = "Hello".to_string();
                let in0 = in0.as_bytes();
                let mut ou0 = [0u8; 64];
                sha3_512(&mut ou0, in0)
            });
        });
    });
}

criterion_group!(benches, bench_sha3_512,);
criterion_main!(benches);
