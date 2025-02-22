#[allow(unused_imports)]
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Moving Average types for technical analysis.
///
/// The integer representation of this enum is determined by the enabled features:
/// - With feature "int64": Uses i64 representation
/// - With feature "int32": Uses i32 representation
/// - With no features enabled: Defaults to i32 representation
///
/// # Variants
///
/// * `DEMA` - Double Exponential Moving Average
/// * `EMA` - Exponential Moving Average
/// * `KAMA` - Kaufman Adaptive Moving Average
/// * `MAMA` - MESA Adaptive Moving Average
/// * `RMA` - Wilder's Smoothed Moving Average
/// * `SMA` - Simple Moving Average
/// * `T3` - Triple Exponential Moving Average (T3)
/// * `TEMA` - Triple Exponential Moving Average
/// * `TRIMA` - Triangular Moving Average
/// * `WMA` - Weighted Moving Average
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg(all(feature = "int64", not(feature = "int32")))]
#[repr(i64)]
pub enum MAType {
    DEMA  = 0,
    EMA   = 1,
    KAMA  = 2,
    MAMA  = 3,
    RMA   = 4,
    SMA   = 5,
    T3    = 6,
    TEMA  = 7,
    TRIMA = 8,
    WMA   = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg(all(feature = "int32", not(feature = "int64")))]
#[repr(i32)]
pub enum MAType {
    DEMA  = 0,
    EMA   = 1,
    KAMA  = 2,
    MAMA  = 3,
    RMA   = 4,
    SMA   = 5,
    T3    = 6,
    TEMA  = 7,
    TRIMA = 8,
    WMA   = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg(not(any(feature = "int32", feature = "int64")))]
#[repr(i32)]
pub enum MAType {
    DEMA  = 0,
    EMA   = 1,
    KAMA  = 2,
    MAMA  = 3,
    RMA   = 4,
    SMA   = 5,
    T3    = 6,
    TEMA  = 7,
    TRIMA = 8,
    WMA   = 9,
}

impl Default for MAType {
    /// Returns the default Moving Average type (SMA).
    ///
    /// # Returns
    ///
    /// * [`MAType::SMA`] - Simple Moving Average, chosen for its:
    ///     - Widespread use and familiarity
    ///     - Computational simplicity
    ///     - Reliable baseline performance
    fn default() -> Self {
        Self::SMA
    }
}

/// Standard signal values for technical indicators.
///
/// The integer representation of this enum is determined by the enabled features:
/// - With feature "int64": Uses i64 representation
/// - With feature "int32": Uses i32 representation
/// - With no features enabled: Defaults to i32 representation
///
/// # Values
/// * `Bullish` (+100): Strong upward potential
/// * `Balance` (50): Moderate signal strength
/// * `Bearish` (-100): Strong downward potential
/// * `Neutral` (0): No clear directional bias
/// * `Pattern` (1): Technical pattern detected
/// * `Invalid` (-1): Invalid value
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg(all(feature = "int64", not(feature = "int32")))]
#[repr(i64)]
pub enum Signal {
    Bullish = 100,
    Balance = 50,
    Bearish = -100,
    Neutral = 0,
    Pattern = 1,
    Invalid = -1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg(all(feature = "int32", not(feature = "int64")))]
#[repr(i32)]
pub enum Signal {
    Bullish = 100,
    Balance = 50,
    Bearish = -100,
    Neutral = 0,
    Pattern = 1,
    Invalid = -1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[cfg(not(any(feature = "int32", feature = "int64")))]
#[repr(i32)]
pub enum Signal {
    Bullish = 100,
    Balance = 50,
    Bearish = -100,
    Neutral = 0,
    Pattern = 1,
    Invalid = -1,
}

impl Default for Signal {
    /// Returns the default signal value (Neutral).
    ///
    /// # Returns
    /// * [`Signal::Neutral`] - Represents no directional bias (0)
    fn default() -> Self {
        Self::Neutral
    }
}
