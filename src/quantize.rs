//! Traits for quantization.
//!
//! Quantization is the process of converting a normalized floating point value
//! (i.e. between 0.0 and 1.0) into a value of another type, losing precision.
//! Dequantization is the opposite; it takes the quantized value and gives the original value back,
//! probably with some error.

/// Trait for quantizing and dequantizing values.
///
/// Responsible for converting a number into another type (quantizing)
/// and back again (dequantizing).
/// The input value is a floating point value between 0.0 and 1.0.
pub trait Quantize {
    /// The type to store the quantized value in.
    type Type;

    /// Quantize a floating point value.
    /// The input value should be between 0.0 and 1.0 inclusive.
    /// Values outside this range will be clamped to the minimum or maximum value.
    fn quantize(value: f64) -> Self::Type;

    /// Restore the floating point value from a quantized value.
    fn dequantize(quantized: Self::Type) -> f64;

    /// Calculates the maximum quantization error.
    /// The absolute difference between the quantized and original value will not be greater than this,
    /// unless it has been clamped because it was out of range.
    fn max_error() -> f64;
}
