use criterion::{BenchmarkId, Criterion, black_box, criterion_group};
use kand::ohlcv::cdl_hammer::cdl_hammer;

use crate::helper::generate_test_data;
#[allow(dead_code)]
fn bench_cdl_hammer(c: &mut Criterion) {
    let mut group = c.benchmark_group("cdl_hammer");

    let sizes = vec![100_000, 1_000_000, 10_000_000];

    for size in sizes {
        let input_open = generate_test_data(size);
        let input_high = generate_test_data(size);
        let input_low = generate_test_data(size);
        let input_close = generate_test_data(size);
        let mut output_signals = vec![0; size];
        let mut output_body_avg = vec![0.0; size];

        group.bench_with_input(BenchmarkId::new("size", size), &size, |b, &_size| {
            b.iter(|| {
                let _ = cdl_hammer(
                    black_box(&input_open),
                    black_box(&input_high),
                    black_box(&input_low),
                    black_box(&input_close),
                    black_box(5),
                    black_box(0.1),
                    black_box(&mut output_signals),
                    black_box(&mut output_body_avg),
                );
            });
        });
    }

    group.finish();
}

criterion_group!(ohlcv, bench_cdl_hammer);
