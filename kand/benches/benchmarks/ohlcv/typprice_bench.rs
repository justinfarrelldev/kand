use criterion::{BenchmarkId, Criterion, black_box, criterion_group};
use kand::ohlcv::typprice::typprice;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_typprice(c: &mut Criterion) {
    let mut group = c.benchmark_group("typprice");

    // Test different data sizes
    let sizes = vec![100_000, 1_000_000, 10_000_000];

    for size in sizes {
        let input_high = generate_test_data(size);
        let input_low = generate_test_data(size);
        let input_close = generate_test_data(size);
        let mut output = vec![0.0; size];

        group.bench_with_input(BenchmarkId::new("size", size), &size, |b, &_size| {
            b.iter(|| {
                let _ = typprice(
                    black_box(&input_high),
                    black_box(&input_low),
                    black_box(&input_close),
                    black_box(&mut output),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(ohlcv, bench_typprice);
