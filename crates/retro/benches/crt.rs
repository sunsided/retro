use criterion::{Criterion, criterion_group, criterion_main};
use retro;
use retro::render::CrtEffect;
use retro::{FrameBuffer, HEIGHT, WIDTH};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let mut fb: FrameBuffer = [0; WIDTH * HEIGHT];
    let mut crt = CrtEffect::new();
    c.bench_function("crt", |b| {
        b.iter(|| {
            crt.render(black_box(&mut fb), black_box(1));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
