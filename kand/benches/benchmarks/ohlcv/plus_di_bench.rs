use criterion::{BenchmarkId, Criterion, black_box, criterion_group};
use kand::ohlcv::plus_di::plus_di;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_plus_di(c: &mut Criterion) {
    let mut group = c.benchmark_group("plus_di");

    // Test different data sizes
    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let periods = vec![5, 50, 200];

    for size in sizes {
        let high = generate_test_data(size);
        let low = generate_test_data(size);
        let close = generate_test_data(size);
        let mut output_plus_di = vec![0.0; size];
        let mut output_smoothed_plus_dm = vec![0.0; size];
        let mut output_smoothed_tr = vec![0.0; size];

        for period in &periods {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{size}"), period),
                period,
                |b, &period| {
                    b.iter(|| {
                        let _ = plus_di(
                            black_box(&high),
                            black_box(&low),
                            black_box(&close),
                            black_box(period),
                            black_box(&mut output_plus_di),
                            black_box(&mut output_smoothed_plus_dm),
                            black_box(&mut output_smoothed_tr),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_plus_di);
