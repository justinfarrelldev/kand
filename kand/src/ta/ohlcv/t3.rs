use num_traits::{Float, FromPrimitive};

use crate::{error::KandError, helper::period_to_k};

/// Calculates the lookback period required for T3 indicator
///
/// # Description
/// Returns the number of data points needed before the first valid T3 value can be calculated.
/// The lookback period is 6 * (period - 1) due to the cascading EMA calculations.
///
/// # Arguments
/// * `param_period` - The smoothing period for the EMAs (must be >= 2)
/// * `param_vfactor` - The volume factor parameter (typically between 0 and 1)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period if successful
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` < 2 (when "check" feature enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::t3;
/// let lookback = t3::lookback(5, 0.7).unwrap();
/// assert_eq!(lookback, 24); // 6 * (5-1) = 24
/// ```
pub const fn lookback(param_period: usize, _param_vfactor: f64) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(6 * (param_period - 1))
}

/// Calculates T3 (Triple Exponential Moving Average) indicator for a price series
///
/// # Description
/// T3 is a sophisticated moving average developed by Tim Tillson that reduces lag while maintaining smoothness.
/// It combines six EMAs with optimized weightings to produce a responsive yet smooth indicator.
///
/// # Mathematical Formula
/// ```text
/// k = 2/(period + 1)
/// EMA1 = EMA(price, period)
/// EMA2 = EMA(EMA1, period)
/// EMA3 = EMA(EMA2, period)
/// EMA4 = EMA(EMA3, period)
/// EMA5 = EMA(EMA4, period)
/// EMA6 = EMA(EMA5, period)
///
/// Coefficients:
/// a = volume_factor
/// c1 = -a^3
/// c2 = 3*a^2 + 3*a^3
/// c3 = -6*a^2 - 3*a - 3*a^3
/// c4 = 1 + 3*a + a^3 + 3*a^2
///
/// T3 = c1*EMA6 + c2*EMA5 + c3*EMA4 + c4*EMA3
/// ```
///
/// # Arguments
/// * `input` - Price data array
/// * `param_period` - Smoothing period for EMAs (must be >= 2)
/// * `param_vfactor` - Volume factor controlling smoothing (typically 0-1)
/// * `output` - Array to store T3 values
/// * `output_ema1` - Array to store EMA1 values
/// * `output_ema2` - Array to store EMA2 values
/// * `output_ema3` - Array to store EMA3 values
/// * `output_ema4` - Array to store EMA4 values
/// * `output_ema5` - Array to store EMA5 values
/// * `output_ema6` - Array to store EMA6 values
///
/// # Returns
/// * `Result<(), KandError>` - Empty Ok if successful
///
/// # Errors
/// * `KandError::InvalidData` - If input array is empty
/// * `KandError::LengthMismatch` - If input/output array lengths don't match
/// * `KandError::InvalidParameter` - If `param_period` < 2
/// * `KandError::InsufficientData` - If input length <= lookback period
/// * `KandError::NaNDetected` - If NaN values found in input
///
/// # Example
/// ```
/// use kand::ohlcv::t3;
///
/// let input = vec![10.0, 12.0, 15.0, 14.0, 13.0, 11.0, 11.0];
/// let mut output = vec![0.0; input.len()];
/// let mut ema1 = vec![0.0; input.len()];
/// let mut ema2 = vec![0.0; input.len()];
/// let mut ema3 = vec![0.0; input.len()];
/// let mut ema4 = vec![0.0; input.len()];
/// let mut ema5 = vec![0.0; input.len()];
/// let mut ema6 = vec![0.0; input.len()];
///
/// t3::t3(
///     &input,
///     2,
///     0.7,
///     &mut output,
///     &mut ema1,
///     &mut ema2,
///     &mut ema3,
///     &mut ema4,
///     &mut ema5,
///     &mut ema6,
/// )
/// .unwrap();
/// ```
pub fn t3<T>(
    input: &[T],
    param_period: usize,
    param_vfactor: f64,
    output: &mut [T],
    output_ema1: &mut [T],
    output_ema2: &mut [T],
    output_ema3: &mut [T],
    output_ema4: &mut [T],
    output_ema5: &mut [T],
    output_ema6: &mut [T],
) -> Result<(), KandError>
where
    T: Float + FromPrimitive,
{
    let len = input.len();
    let lookback = lookback(param_period, param_vfactor)?;

    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Data sufficiency check
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }

        // Length consistency check
        if len != output.len()
            || len != output_ema1.len()
            || len != output_ema2.len()
            || len != output_ema3.len()
            || len != output_ema4.len()
            || len != output_ema5.len()
            || len != output_ema6.len()
        {
            return Err(KandError::LengthMismatch);
        }

        // Parameter range check
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        for value in input.iter().take(len) {
            if value.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    let a = T::from(param_vfactor).ok_or(KandError::ConversionError)?;
    let a2 = a * a;
    let a3 = a2 * a;

    // Calculate coefficients consistent with TA-Lib
    let c1 = -a3;
    let c2 = T::from(3.0).ok_or(KandError::ConversionError)? * (a2 + a3);
    let c3 = -T::from(6.0).ok_or(KandError::ConversionError)? * a2
        - T::from(3.0).ok_or(KandError::ConversionError)? * a
        - T::from(3.0).ok_or(KandError::ConversionError)? * a3;
    let c4 = T::one()
        + T::from(3.0).ok_or(KandError::ConversionError)? * a
        + a3
        + T::from(3.0).ok_or(KandError::ConversionError)? * a2;

    let k = period_to_k(param_period)?;
    let one_minus_k = T::one() - k;

    // Sequential initialization mimicking TA-Lib's warm-up process.
    // "today" will serve as our pointer into the input array.
    let mut today = 0;

    // Initialize EMA1 with a simple moving average (SMA) of the first 'param_period' values.
    let mut temp = T::zero();
    for _ in 0..param_period {
        temp = temp + input[today];
        today += 1;
    }
    let mut e1 = temp / T::from(param_period).ok_or(KandError::ConversionError)?;

    // Initialize EMA2 using the next (param_period - 1) values.
    temp = e1;
    for _ in 1..param_period {
        e1 = k * input[today] + one_minus_k * e1;
        temp = temp + e1;
        today += 1;
    }
    let mut e2 = temp / T::from(param_period).ok_or(KandError::ConversionError)?;

    // Initialize EMA3.
    temp = e2;
    for _ in 1..param_period {
        e1 = k * input[today] + one_minus_k * e1;
        e2 = k * e1 + one_minus_k * e2;
        temp = temp + e2;
        today += 1;
    }
    let mut e3 = temp / T::from(param_period).ok_or(KandError::ConversionError)?;

    // Initialize EMA4.
    temp = e3;
    for _ in 1..param_period {
        e1 = k * input[today] + one_minus_k * e1;
        e2 = k * e1 + one_minus_k * e2;
        e3 = k * e2 + one_minus_k * e3;
        temp = temp + e3;
        today += 1;
    }
    let mut e4 = temp / T::from(param_period).ok_or(KandError::ConversionError)?;

    // Initialize EMA5.
    temp = e4;
    for _ in 1..param_period {
        e1 = k * input[today] + one_minus_k * e1;
        e2 = k * e1 + one_minus_k * e2;
        e3 = k * e2 + one_minus_k * e3;
        e4 = k * e3 + one_minus_k * e4;
        temp = temp + e4;
        today += 1;
    }
    let mut e5 = temp / T::from(param_period).ok_or(KandError::ConversionError)?;

    // Initialize EMA6.
    temp = e5;
    for _ in 1..param_period {
        e1 = k * input[today] + one_minus_k * e1;
        e2 = k * e1 + one_minus_k * e2;
        e3 = k * e2 + one_minus_k * e3;
        e4 = k * e3 + one_minus_k * e4;
        e5 = k * e4 + one_minus_k * e5;
        temp = temp + e5;
        today += 1;
    }
    let mut e6 = temp / T::from(param_period).ok_or(KandError::ConversionError)?;

    // Skip the remainder of the unstable period (if any).
    while today <= lookback {
        e1 = k * input[today] + one_minus_k * e1;
        e2 = k * e1 + one_minus_k * e2;
        e3 = k * e2 + one_minus_k * e3;
        e4 = k * e3 + one_minus_k * e4;
        e5 = k * e4 + one_minus_k * e5;
        e6 = k * e5 + one_minus_k * e6;
        today += 1;
    }

    // Write the first valid output at index = lookback.
    output[lookback] = c1 * e6 + c2 * e5 + c3 * e4 + c4 * e3;
    output_ema1[lookback] = e1;
    output_ema2[lookback] = e2;
    output_ema3[lookback] = e3;
    output_ema4[lookback] = e4;
    output_ema5[lookback] = e5;
    output_ema6[lookback] = e6;

    // Process the remaining data points.
    while today < len {
        e1 = k * input[today] + one_minus_k * e1;
        e2 = k * e1 + one_minus_k * e2;
        e3 = k * e2 + one_minus_k * e3;
        e4 = k * e3 + one_minus_k * e4;
        e5 = k * e4 + one_minus_k * e5;
        e6 = k * e5 + one_minus_k * e6;

        output[today] = c1 * e6 + c2 * e5 + c3 * e4 + c4 * e3;
        output_ema1[today] = e1;
        output_ema2[today] = e2;
        output_ema3[today] = e3;
        output_ema4[today] = e4;
        output_ema5[today] = e5;
        output_ema6[today] = e6;
        today += 1;
    }

    // Mark the unstable period indices with NaN.
    for i in 0..lookback {
        output[i] = T::nan();
        output_ema1[i] = T::nan();
        output_ema2[i] = T::nan();
        output_ema3[i] = T::nan();
        output_ema4[i] = T::nan();
        output_ema5[i] = T::nan();
        output_ema6[i] = T::nan();
    }

    Ok(())
}

/// Calculates the latest T3 value incrementally using previous EMA values
///
/// # Description
/// Provides an efficient way to update T3 values in real-time by using previously calculated EMA values.
/// This avoids recalculating the entire series when only the latest value is needed.
///
/// # Arguments
/// * `input_price` - Latest price value to calculate T3 from
/// * `prev_ema1` - Previous EMA1 value
/// * `prev_ema2` - Previous EMA2 value
/// * `prev_ema3` - Previous EMA3 value
/// * `prev_ema4` - Previous EMA4 value
/// * `prev_ema5` - Previous EMA5 value
/// * `prev_ema6` - Previous EMA6 value
/// * `param_period` - Smoothing period for EMAs (must be >= 2)
/// * `param_vfactor` - Volume factor (typically 0-1)
///
/// # Returns
/// * `Result<(T, T, T, T, T, T, T), KandError>` - Tuple containing:
///   - Latest T3 value
///   - Updated EMA1 value
///   - Updated EMA2 value
///   - Updated EMA3 value
///   - Updated EMA4 value
///   - Updated EMA5 value
///   - Updated EMA6 value
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` < 2
/// * `KandError::NaNDetected` - If any input value is NaN
///
/// # Example
/// ```
/// use kand::ohlcv::t3;
///
/// let (t3_value, ema1, ema2, ema3, ema4, ema5, ema6) = t3::t3_incremental(
///     100.0, // New price
///     95.0,  // Previous EMA1
///     94.0,  // Previous EMA2
///     93.0,  // Previous EMA3
///     92.0,  // Previous EMA4
///     91.0,  // Previous EMA5
///     90.0,  // Previous EMA6
///     5,     // Period
///     0.7,   // Volume factor
/// )
/// .unwrap();
/// ```
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn t3_incremental<T>(
    input_price: T,
    prev_ema1: T,
    prev_ema2: T,
    prev_ema3: T,
    prev_ema4: T,
    prev_ema5: T,
    prev_ema6: T,
    param_period: usize,
    param_vfactor: f64,
) -> Result<(T, T, T, T, T, T, T), KandError>
where
    T: Float + FromPrimitive,
{
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        if input_price.is_nan()
            || prev_ema1.is_nan()
            || prev_ema2.is_nan()
            || prev_ema3.is_nan()
            || prev_ema4.is_nan()
            || prev_ema5.is_nan()
            || prev_ema6.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let k = period_to_k(param_period)?;
    let one_minus_k = T::one() - k;

    // Calculate new EMA values
    let ema1 = input_price * k + prev_ema1 * one_minus_k;
    let ema2 = ema1 * k + prev_ema2 * one_minus_k;
    let ema3 = ema2 * k + prev_ema3 * one_minus_k;
    let ema4 = ema3 * k + prev_ema4 * one_minus_k;
    let ema5 = ema4 * k + prev_ema5 * one_minus_k;
    let ema6 = ema5 * k + prev_ema6 * one_minus_k;

    // Calculate coefficients
    let a = T::from(param_vfactor).ok_or(KandError::ConversionError)?;
    let a2 = a * a;
    let a3 = a2 * a;

    let c1 = -a3;
    let c2 = T::from(3.0).ok_or(KandError::ConversionError)? * (a2 + a3);
    let c3 = -T::from(6.0).ok_or(KandError::ConversionError)? * a2
        - T::from(3.0).ok_or(KandError::ConversionError)? * a
        - T::from(3.0).ok_or(KandError::ConversionError)? * a3;
    let c4 = T::one()
        + T::from(3.0).ok_or(KandError::ConversionError)? * a
        + a3
        + T::from(3.0).ok_or(KandError::ConversionError)? * a2;

    // Calculate T3
    let t3 = c1 * ema6 + c2 * ema5 + c3 * ema4 + c4 * ema3;

    Ok((t3, ema1, ema2, ema3, ema4, ema5, ema6))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_t3_calculation() {
        let input = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0, 35114.5, 35097.2,
            35092.0, 35073.2, 35139.3, 35092.0, 35126.7, 35106.3, 35124.8, 35170.1,
        ];

        let param_period = 5;
        let param_vfactor = 0.7;
        let mut output = vec![0.0; input.len()];
        let mut output_ema1 = vec![0.0; input.len()];
        let mut output_ema2 = vec![0.0; input.len()];
        let mut output_ema3 = vec![0.0; input.len()];
        let mut output_ema4 = vec![0.0; input.len()];
        let mut output_ema5 = vec![0.0; input.len()];
        let mut output_ema6 = vec![0.0; input.len()];

        // Calculate T3 for the full series
        t3(
            &input,
            param_period,
            param_vfactor,
            &mut output,
            &mut output_ema1,
            &mut output_ema2,
            &mut output_ema3,
            &mut output_ema4,
            &mut output_ema5,
            &mut output_ema6,
        )
        .unwrap();

        // First 24 values should be NaN (lookback = 6 * (period - 1) = 24)
        for value in output.iter().take(24) {
            assert!(value.is_nan());
        }

        // Compare with known values
        let expected_values = [
            34_990.330_892_685_28,
            35_014.010_406_572_81,
            35_039.640_085_147_26,
            35_061.489_687_430_236,
            35_075.563_406_161_85,
            35_090.473_558_789_85,
            35_100.366_627_049_894,
            35_109.086_698_426_225,
            35_114.435_788_643_98,
            35_118.848_638_076_015,
            35_127.887_566_614_78,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output[i + 24], *expected, epsilon = 0.0001);
        }

        // Test incremental calculation
        let mut prev_ema1: f64 = output_ema1[24];
        let mut prev_ema2: f64 = output_ema2[24];
        let mut prev_ema3: f64 = output_ema3[24];
        let mut prev_ema4: f64 = output_ema4[24];
        let mut prev_ema5: f64 = output_ema5[24];
        let mut prev_ema6: f64 = output_ema6[24];

        // Test each incremental step matches the full calculation
        for i in 25..input.len() {
            let (t3, ema1, ema2, ema3, ema4, ema5, ema6) = t3_incremental(
                input[i],
                prev_ema1,
                prev_ema2,
                prev_ema3,
                prev_ema4,
                prev_ema5,
                prev_ema6,
                param_period,
                param_vfactor,
            )
            .unwrap();

            assert_relative_eq!(t3, output[i], epsilon = 0.0001);
            assert_relative_eq!(ema1, output_ema1[i], epsilon = 0.0001);
            assert_relative_eq!(ema2, output_ema2[i], epsilon = 0.0001);
            assert_relative_eq!(ema3, output_ema3[i], epsilon = 0.0001);
            assert_relative_eq!(ema4, output_ema4[i], epsilon = 0.0001);
            assert_relative_eq!(ema5, output_ema5[i], epsilon = 0.0001);
            assert_relative_eq!(ema6, output_ema6[i], epsilon = 0.0001);

            prev_ema1 = ema1;
            prev_ema2 = ema2;
            prev_ema3 = ema3;
            prev_ema4 = ema4;
            prev_ema5 = ema5;
            prev_ema6 = ema6;
        }
    }
}
