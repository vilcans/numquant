//! For quantizing values linearly to a range.

use crate::Quantize;
use std::ops::Range;

/// For quantizing values linearly to a range.
///
/// The range specifies the range of input values this type accepts.
/// The value at the beginning of the range will be quantized to 0.
/// The value at the end of the range will be quantized to `q_max`.
/// Values in-between are linearly interpolated between 0 and `q_max`.
/// Values outside of the range will be clamped to be inside the range.
pub trait Linear {
    type Type;

    /// The minimum and maximum input values that can be quantized.
    fn range() -> Range<f64>;

    /// Maximum for the quantized value.
    /// The quantized value will be between 0 and this value (inclusive).
    fn q_max() -> Self::Type;
}

impl<L: Linear> Quantize for L
where
    <L as Linear>::Type: Into<f64> + TryFrom<u64> + Copy,
{
    type Type = L::Type;

    fn quantize(value: f64) -> Self::Type {
        quantize(value, Self::range(), Self::q_max())
    }

    fn dequantize(quantized: Self::Type) -> f64 {
        dequantize(quantized, Self::range(), Self::q_max())
    }

    fn max_error() -> f64 {
        max_error(Self::range(), Self::q_max().into())
    }
}

/// Linearly quantize a value.
pub fn quantize<T>(value: f64, range: Range<f64>, q_max: T) -> T
where
    T: Into<f64> + TryFrom<u64> + Copy,
{
    let normalized = ((value - range.start) / (range.end - range.start)).clamp(0.0, 1.0);
    let q_max: f64 = q_max.into();
    // Regarding the `min` call, see https://stackoverflow.com/a/600016
    let v = ((q_max + 1.0) * normalized).min(q_max) as u64;
    match v.try_into() {
        Ok(v) => v,
        Err(_) => panic!("{v} not convertible to T"),
    }
}

/// Linearly dequantize a value.
pub fn dequantize<T>(quantized: T, range: Range<f64>, q_max: T) -> f64
where
    T: Into<f64> + Copy,
{
    let quantized = quantized.into();
    let q_max = q_max.into();
    range.start + (range.end - range.start) * quantized / q_max
}

/// The maximum error when quantizing linearly.
/// The absolute difference between the quantized and original value will not be greater than this,
/// unless it has been clamped because it was out of range.
pub fn max_error(range: Range<f64>, q_max: f64) -> f64 {
    (range.end - range.start) / (q_max + 1.0)
}
