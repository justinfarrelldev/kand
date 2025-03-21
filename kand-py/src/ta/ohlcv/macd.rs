use kand::{TAFloat, ohlcv::macd};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Moving Average Convergence Divergence (MACD) over a NumPy array.
///
/// MACD is a trend-following momentum indicator that shows the relationship between two moving averages
/// of an asset's price. It consists of three components:
/// - MACD Line: Difference between fast and slow EMAs
/// - Signal Line: EMA of the MACD line
/// - Histogram: Difference between MACD line and signal line
///
/// Args:
///   data: Input price data as a 1-D NumPy array of type `TAFloat`.
///   fast_period: Period for fast EMA calculation (typically 12).
///   slow_period: Period for slow EMA calculation (typically 26).
///   signal_period: Period for signal line calculation (typically 9).
///
/// Returns:
///   A tuple of five 1-D NumPy arrays containing:
///   - MACD line values
///   - Signal line values
///   - MACD histogram values
///   - Fast EMA values
///   - Slow EMA values
///   Each array has the same length as the input, with initial elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> data = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
///   >>> macd_line, signal_line, histogram, fast_ema, slow_ema = kand.macd(data, 2, 3, 2)
///   ```
#[pyfunction]
#[pyo3(name = "macd", signature = (data, fast_period, slow_period, signal_period))]
pub fn macd_py(
    py: Python,
    data: PyReadonlyArray1<TAFloat>,
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    // Convert the input NumPy array to a Rust slice
    let input = data.as_slice()?;
    let len = input.len();

    // Create output arrays using vec
    let mut macd_line = vec![0.0; len];
    let mut signal_line = vec![0.0; len];
    let mut histogram = vec![0.0; len];
    let mut fast_ema = vec![0.0; len];
    let mut slow_ema = vec![0.0; len];

    // Perform MACD calculation while releasing the GIL
    py.allow_threads(|| {
        macd::macd(
            input,
            fast_period,
            slow_period,
            signal_period,
            macd_line.as_mut_slice(),
            signal_line.as_mut_slice(),
            histogram.as_mut_slice(),
            fast_ema.as_mut_slice(),
            slow_ema.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert output arrays to Python objects
    Ok((
        macd_line.into_pyarray(py).into(),
        signal_line.into_pyarray(py).into(),
        histogram.into_pyarray(py).into(),
        fast_ema.into_pyarray(py).into(),
        slow_ema.into_pyarray(py).into(),
    ))
}

/// Computes the latest MACD values incrementally from previous state.
///
/// This function provides an efficient way to calculate MACD for streaming data by using
/// previous EMA values instead of recalculating the entire series.
///
/// Args:
///
///   price: Current price value as `TAFloat`.
///   prev_fast_ema: Previous fast EMA value as `TAFloat`.
///   prev_slow_ema: Previous slow EMA value as `TAFloat`.
///   prev_signal: Previous signal line value as `TAFloat`.
///   fast_period: Period for fast EMA calculation (typically 12).
///   slow_period: Period for slow EMA calculation (typically 26).
///   signal_period: Period for signal line calculation (typically 9).
///
/// Returns:
///   A tuple of three values:
///   - MACD line value
///   - Signal line value
///   - MACD histogram value
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> macd_line, signal_line, histogram = kand.macd_inc(
///   ...     100.0,  # current price
///   ...     95.0,   # previous fast EMA
///   ...     98.0,   # previous slow EMA
///   ...     -2.5,   # previous signal
///   ...     12,     # fast period
///   ...     26,     # slow period
///   ...     9       # signal period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "macd_inc", signature = (price, prev_fast_ema, prev_slow_ema, prev_signal, fast_period, slow_period, signal_period))]
pub fn macd_inc_py(
    py: Python,
    price: TAFloat,
    prev_fast_ema: TAFloat,
    prev_slow_ema: TAFloat,
    prev_signal: TAFloat,
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat)> {
    // Perform incremental MACD calculation while releasing the GIL
    py.allow_threads(|| {
        macd::macd_inc(
            price,
            prev_fast_ema,
            prev_slow_ema,
            prev_signal,
            fast_period,
            slow_period,
            signal_period,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
