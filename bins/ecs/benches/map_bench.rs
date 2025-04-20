// benches/map_bench.rs

use ahash::HashMapExt;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use dashmap::DashMap;
use fxhash::FxHashMap;
use fxhash::FxHasher;
use std::hash::BuildHasherDefault;

// Alias for using FxHasher in DashMap
type FxBuildHasher = BuildHasherDefault<FxHasher>;

// Number of keys to insert / lookup
const N: usize = 100_000;

// Generate a Vec<u64> of keys 0..N
fn make_keys() -> Vec<u64> {
    (0..N as u64).collect()
}

fn bench_insert(c: &mut Criterion) {
    let keys = make_keys();
    let mut group = c.benchmark_group("map_insert");

    // 1) Pure FxHashMap
    group.bench_function("FxHashMap", |b| {
        b.iter_batched(
            || keys.clone(),
            |data| {
                let mut m = FxHashMap::with_capacity(N);
                for &k in &data {
                    m.insert(k, k);
                }
            },
            BatchSize::LargeInput,
        )
    });

    // 2) DashMap with default hasher (ahash)
    group.bench_function("DashMap<default>", |b| {
        b.iter_batched(
            || keys.clone(),
            |data| {
                let m: DashMap<u64, u64> = DashMap::with_capacity(N);
                for &k in &data {
                    m.insert(k, k);
                }
            },
            BatchSize::LargeInput,
        )
    });

    // 3) DashMap with FxHasher
    group.bench_function("DashMap<fxhash>", |b| {
        b.iter_batched(
            || keys.clone(),
            |data| {
                let m: DashMap<u64, u64, FxBuildHasher> =
                    DashMap::with_hasher(FxBuildHasher::default());
                for &k in &data {
                    m.insert(k, k);
                }
            },
            BatchSize::LargeInput,
        )
    });

    group.finish();
}

fn bench_read(c: &mut Criterion) {
    let keys = make_keys();
    let mut group = c.benchmark_group("map_read");

    // 1) Pure FxHashMap
    group.bench_function("FxHashMap get", |b| {
        b.iter_batched(
            || {
                let mut m = FxHashMap::with_capacity(N);
                for &k in &keys {
                    m.insert(k, k);
                }
                (m, keys.clone())
            },
            |(mut m, data)| {
                for &k in &data {
                    // we know they exist
                    let _ = m.get(&k);
                }
            },
            BatchSize::LargeInput,
        )
    });

    // 2) DashMap with default hasher
    group.bench_function("DashMap<default> get", |b| {
        b.iter_batched(
            || {
                let m: DashMap<u64, u64> = DashMap::with_capacity(N);
                for &k in &keys {
                    m.insert(k, k);
                }
                (m, keys.clone())
            },
            |(m, data)| {
                for &k in &data {
                    let _ = m.get(&k);
                }
            },
            BatchSize::LargeInput,
        )
    });

    // 3) DashMap with FxHasher
    group.bench_function("DashMap<fxhash> get", |b| {
        b.iter_batched(
            || {
                let m: DashMap<u64, u64, FxBuildHasher> =
                    DashMap::with_hasher(FxBuildHasher::default());
                for &k in &keys {
                    m.insert(k, k);
                }
                (m, keys.clone())
            },
            |(m, data)| {
                for &k in &data {
                    let _ = m.get(&k);
                }
            },
            BatchSize::LargeInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_insert, bench_read);
criterion_main!(benches);
