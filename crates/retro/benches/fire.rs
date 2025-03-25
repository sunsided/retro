use criterion::{Criterion, criterion_group, criterion_main};
use retro;
use retro::render::Fire;
use retro::{FrameBuffer, HEIGHT, WIDTH};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let mut fb: FrameBuffer = [0; WIDTH * HEIGHT];
    let mut fire = Fire::new();
    c.bench_function("fire::update", |b| {
        b.iter(|| {
            fire.update();
        })
    });

    c.bench_function("fire::render", |b| {
        b.iter(|| {
            fire.render(black_box(&mut fb));
        })
    });

    c.bench_function("fire", |b| {
        b.iter(|| {
            fire.update();
            fire.render(black_box(&mut fb));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
