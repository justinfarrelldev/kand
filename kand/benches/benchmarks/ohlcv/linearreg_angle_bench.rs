use criterion::{black_box, criterion_group, BenchmarkId, Criterion};
use kand::ohlcv::linearreg_angle::linearreg_angle;

use crate::helpers::generate_test_data;

#[allow(dead_code)]
fn bench_linearreg_angle(c: &mut Criterion) {
    let mut group = c.benchmark_group("linearreg_angle");

    // Test different data sizes
    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let periods = vec![5, 50, 200];

    for size in sizes {
        let input = generate_test_data(size);
        let mut output = vec![0.0; size];

        for period in &periods {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{}", size), period),
                period,
                |b, &period| {
                    b.iter(|| {
                        let _ = linearreg_angle(black_box(&input), black_box(period), black_box(&mut output));
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(ohlcv, bench_linearreg_angle);
