use criterion::{BenchmarkId, Criterion, black_box, criterion_group};
use kand::ohlcv::obv::obv;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_obv(c: &mut Criterion) {
    let mut group = c.benchmark_group("obv");

    // Test different data sizes
    let sizes = vec![100_000, 1_000_000, 10_000_000];

    for size in sizes {
        let close = generate_test_data(size);
        let volume = generate_test_data(size);
        let mut output = vec![0.0; size];

        group.bench_with_input(BenchmarkId::new("size", size), &size, |b, &_size| {
            b.iter(|| {
                let _ = obv(
                    black_box(&close),
                    black_box(&volume),
                    black_box(&mut output),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(ohlcv, bench_obv);
