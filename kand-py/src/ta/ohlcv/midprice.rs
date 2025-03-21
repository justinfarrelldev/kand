use kand::{TAFloat, ohlcv::midprice};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculates Midpoint Price values for a NumPy array.
///
/// The Midpoint Price is a technical indicator that represents the mean value between the highest high
/// and lowest low prices over a specified period.
///
/// Args:
///   high: Input high price data as a 1-D NumPy array of type `TAFloat`.
///   low: Input low price data as a 1-D NumPy array of type `TAFloat`.
///   period: Time period for calculation (must be >= 2).
///
/// Returns:
///   A tuple of three 1-D NumPy arrays containing:
///   - Midpoint Price values
///   - Highest high values for each period
///   - Lowest low values for each period
///   Each array has the same length as the input, with initial elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
///   >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
///   >>> midprice, highest, lowest = kand.midprice(high, low, 3)
///   ```
#[pyfunction]
#[pyo3(name = "midprice", signature = (high, low, period))]
pub fn midprice_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    let input_high = high.as_slice()?;
    let input_low = low.as_slice()?;
    let len = input_high.len();

    let mut output_midprice = vec![0.0; len];
    let mut output_highest = vec![0.0; len];
    let mut output_lowest = vec![0.0; len];

    py.allow_threads(|| {
        midprice::midprice(
            input_high,
            input_low,
            period,
            output_midprice.as_mut_slice(),
            output_highest.as_mut_slice(),
            output_lowest.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_midprice.into_pyarray(py).into(),
        output_highest.into_pyarray(py).into(),
        output_lowest.into_pyarray(py).into(),
    ))
}

/// Calculates the next Midpoint Price value incrementally.
///
/// Provides an optimized way to calculate the next Midpoint Price value when new data arrives,
/// without recalculating the entire series.
///
/// Args:
///
///   high: Current high price value as `TAFloat`.
///   low: Current low price value as `TAFloat`.
///   prev_highest: Previous highest high value as `TAFloat`.
///   prev_lowest: Previous lowest low value as `TAFloat`.
///   period: Time period for calculation (must be >= 2).
///
/// Returns:
///   A tuple containing:
///   - Midpoint Price value
///   - New highest high value
///   - New lowest low value
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> midprice, new_highest, new_lowest = kand.midprice_inc(
///   ...     10.5,  # current high
///   ...     9.8,   # current low
///   ...     10.2,  # previous highest high
///   ...     9.5,   # previous lowest low
///   ...     14     # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "midprice_inc", signature = (high, low, prev_highest, prev_lowest, period))]
pub fn midprice_inc_py(
    py: Python,
    high: TAFloat,
    low: TAFloat,
    prev_highest: TAFloat,
    prev_lowest: TAFloat,
    period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat)> {
    py.allow_threads(|| midprice::midprice_inc(high, low, prev_highest, prev_lowest, period))
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
