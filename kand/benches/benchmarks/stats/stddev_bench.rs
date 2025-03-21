use criterion::{BenchmarkId, Criterion, black_box, criterion_group};
use kand::stats::stddev::stddev;

use crate::helper::generate_test_data;

#[allow(dead_code)]
fn bench_stddev(c: &mut Criterion) {
    let mut group = c.benchmark_group("stddev");

    // Test different data sizes
    let sizes = vec![100_000, 1_000_000, 10_000_000];
    let periods = vec![5, 50, 200];

    for size in sizes {
        let input = generate_test_data(size);
        let mut output = vec![0.0; size];
        let mut output_sum = vec![0.0; size];
        let mut output_sum_sq = vec![0.0; size];

        for period in &periods {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{size}"), period),
                period,
                |b, &period| {
                    b.iter(|| {
                        let _ = stddev(
                            black_box(&input),
                            black_box(period),
                            black_box(&mut output),
                            black_box(&mut output_sum),
                            black_box(&mut output_sum_sq),
                        );
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group!(stats, bench_stddev);
