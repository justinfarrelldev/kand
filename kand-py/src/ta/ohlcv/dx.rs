use kand::{ohlcv::dx, TAFloat};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Directional Movement Index (DX) over NumPy arrays.
///
/// The DX indicator measures the strength of a trend by comparing positive and negative directional movements.
///
/// Args:
///   py: Python interpreter token required for GIL management.
///   high: High prices as a 1-D NumPy array of type `f32`.
///   low: Low prices as a 1-D NumPy array of type `f32`.
///   close: Close prices as a 1-D NumPy array of type `f32`.
///   period: Window size for DX calculation. Must be positive and less than input length.
///
/// Returns:
///   A tuple of four 1-D NumPy arrays containing:
///   - DX values
///   - Smoothed +DM values
///   - Smoothed -DM values
///   - Smoothed TR values
///   Each array has the same length as the input, with the first `period` elements containing NaN values.
///
/// Note:
///   This function releases the Python GIL during computation using `py.allow_threads()` to enable
///   concurrent Python execution.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([24.20, 24.07, 24.04, 23.87, 23.67])
///   >>> low = np.array([23.85, 23.72, 23.64, 23.37, 23.46])
///   >>> close = np.array([23.89, 23.95, 23.67, 23.78, 23.50])
///   >>> dx, plus_dm, minus_dm, tr = kand.dx(high, low, close, 3)
///   ```
#[pyfunction]
#[pyo3(name = "dx", signature = (high, low, close, period))]
pub fn dx_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    // Convert the input NumPy arrays to Rust slices
    let high_input = high.as_slice()?;
    let low_input = low.as_slice()?;
    let close_input = close.as_slice()?;
    let len = high_input.len();

    // Create new output arrays using vec
    let mut output_dx = vec![0.0; len];
    let mut output_smoothed_plus_dm = vec![0.0; len];
    let mut output_smoothed_minus_dm = vec![0.0; len];
    let mut output_smoothed_tr = vec![0.0; len];

    // Perform the DX calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| {
        dx::dx(
            high_input,
            low_input,
            close_input,
            period,
            output_dx.as_mut_slice(),
            output_smoothed_plus_dm.as_mut_slice(),
            output_smoothed_minus_dm.as_mut_slice(),
            output_smoothed_tr.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output arrays to Python objects
    Ok((
        output_dx.into_pyarray(py).into(),
        output_smoothed_plus_dm.into_pyarray(py).into(),
        output_smoothed_minus_dm.into_pyarray(py).into(),
        output_smoothed_tr.into_pyarray(py).into(),
    ))
}

/// Calculate the latest DX value incrementally
///
/// # Description
/// Calculates only the most recent DX value using previous smoothed values.
/// This is optimized for real-time calculations where only the latest value is needed.
///
/// # Formula
/// See the formula section in the [`dx`] function documentation.
///
/// # Arguments
/// * `input_high` - Current high price
/// * `input_low` - Current low price
/// * `input_prev_high` - Previous period's high price
/// * `input_prev_low` - Previous period's low price
/// * `input_prev_close` - Previous period's close price
/// * `input_prev_smoothed_plus_dm` - Previous smoothed +DM value
/// * `input_prev_smoothed_minus_dm` - Previous smoothed -DM value
/// * `input_prev_smoothed_tr` - Previous smoothed TR value
/// * `param_period` - Period for DX calculation (typically 14)
///
/// # Returns
/// * `PyResult<(f32, f32, f32, f32)>` - Tuple containing:
///   - Latest DX value
///   - New smoothed +DM
///   - New smoothed -DM
///   - New smoothed TR
///
/// # Example
/// ```python
/// >>> import kand
/// >>> high = 24.20
/// >>> low = 23.85
/// >>> prev_high = 24.07
/// >>> prev_low = 23.72
/// >>> prev_close = 23.95
/// >>> prev_smoothed_plus_dm = 0.5
/// >>> prev_smoothed_minus_dm = 0.3
/// >>> prev_smoothed_tr = 1.2
/// >>> period = 14
/// >>> dx, plus_dm, minus_dm, tr = kand.dx_incremental(
/// ...     high, low, prev_high, prev_low, prev_close,
/// ...     prev_smoothed_plus_dm, prev_smoothed_minus_dm,
/// ...     prev_smoothed_tr, period)
/// ```
#[pyfunction]
#[pyo3(name = "dx_incremental", signature = (
    input_high,
    input_low,
    input_prev_high,
    input_prev_low,
    input_prev_close,
    input_prev_smoothed_plus_dm,
    input_prev_smoothed_minus_dm,
    input_prev_smoothed_tr,
    param_period
))]
pub fn dx_incremental_py(
    py: Python,
    input_high: TAFloat,
    input_low: TAFloat,
    input_prev_high: TAFloat,
    input_prev_low: TAFloat,
    input_prev_close: TAFloat,
    input_prev_smoothed_plus_dm: TAFloat,
    input_prev_smoothed_minus_dm: TAFloat,
    input_prev_smoothed_tr: TAFloat,
    param_period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat, TAFloat)> {
    // Perform the incremental DX calculation while releasing the GIL
    py.allow_threads(|| {
        dx::dx_incremental(
            input_high,
            input_low,
            input_prev_high,
            input_prev_low,
            input_prev_close,
            input_prev_smoothed_plus_dm,
            input_prev_smoothed_minus_dm,
            input_prev_smoothed_tr,
            param_period,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
