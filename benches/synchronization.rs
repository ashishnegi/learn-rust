extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::{Mutex, atomic::AtomicU64, atomic::Ordering};
// use std::time::Instant;

fn add(num: u64) -> u64 {
    num + 1
}

fn atomic_add(num: &AtomicU64) -> u64 {
    num.fetch_add(1, Ordering::SeqCst);
    num.load(Ordering::SeqCst)
}

fn mutex_add(mutex: &Mutex<u64>) -> u64 {
    if let Ok(ref mut m) = mutex.lock() {
        **m += 1;
        return **m;
    }
    return 0;
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("add", |b| b.iter(|| add(10000)));

    c.bench_function("mutex_add", |b| {
        let mutex = Mutex::new(0);
        b.iter(|| mutex_add(&mutex))
    });

    c.bench_function("atomic_add", |b| {
        let num = AtomicU64::new(0);
        b.iter(|| atomic_add(&num))
    });

    // c.bench_function("contented_mutex_add", |b| {
    //     let mutex = Mutex::new(0);

    //     b.iter_custom(|iters| {
    //         let start = Instant::now();
    //         for _i in 0..iters {
    //             add(1000);
    //         }
    //         start.elapsed()
    //     })
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);