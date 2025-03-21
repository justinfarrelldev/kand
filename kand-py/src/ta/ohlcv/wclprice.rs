use kand::{TAFloat, ohlcv::wclprice};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculates the Weighted Close Price (WCLPRICE) for a series of price data.
///
/// The Weighted Close Price is a price indicator that assigns more weight to the closing price
/// compared to high and low prices. It provides a single value that reflects price action
/// with emphasis on the closing price.
///
/// Args:
///   high: High prices as a 1-D NumPy array of type `TAFloat`.
///   low: Low prices as a 1-D NumPy array of type `TAFloat`.
///   close: Close prices as a 1-D NumPy array of type `TAFloat`.
///
/// Returns:
///   A 1-D NumPy array containing the WCLPRICE values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([10.0, 12.0, 15.0])
///   >>> low = np.array([8.0, 9.0, 11.0])
///   >>> close = np.array([9.0, 11.0, 14.0])
///   >>> wclprice = kand.wclprice(high, low, close)
///   ```
#[pyfunction]
#[pyo3(name = "wclprice", signature = (high, low, close))]
pub fn wclprice_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
) -> PyResult<Py<PyArray1<TAFloat>>> {
    let high_slice = high.as_slice()?;
    let low_slice = low.as_slice()?;
    let close_slice = close.as_slice()?;
    let len = high_slice.len();

    let mut output = vec![0.0; len];

    py.allow_threads(|| {
        wclprice::wclprice(high_slice, low_slice, close_slice, output.as_mut_slice())
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok(output.into_pyarray(py).into())
}

/// Calculates a single Weighted Close Price (WCLPRICE) value from the latest price data.
///
/// Args:
///   high: Latest high price value as `TAFloat`.
///   low: Latest low price value as `TAFloat`.
///   close: Latest close price value as `TAFloat`.
///
/// Returns:
///   The calculated WCLPRICE value.
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> wclprice = kand.wclprice_inc(15.0, 11.0, 14.0)
///   ```
#[pyfunction]
#[pyo3(name = "wclprice_inc", signature = (high, low, close))]
pub fn wclprice_inc_py(high: TAFloat, low: TAFloat, close: TAFloat) -> PyResult<TAFloat> {
    wclprice::wclprice_inc(high, low, close)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
