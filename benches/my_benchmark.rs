use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;
use rSentenceHash::unsigned_num_to_hex;

lazy_static! {
    static ref BUFFER: Vec<u8> = vec![0u8; 16];
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("num_to_hex", |b| {
        let mut buffer = [0u8; 16];

        b.iter(|| {
            unsigned_num_to_hex(black_box(48_405_995_369), black_box(&mut buffer))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);