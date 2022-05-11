//! Traits for normalization and denormalization.
//!
//! Normalization is the process of converting a value of any range into a value between 0.0 and 1.0.
//! Denormalization is the opposite; converting the normalized value back to the original range.

/// Normalizes a value into the range 0.0..1.0 and vice versa.
pub trait Normalize {
    /// Convert a value to a floating point value between 0.0 and 1.0.
    fn normalize(value: f64) -> f64;
    /// Convert a floating point value between 0.0 and 1.0 to the range.
    fn denormalize(n: f64) -> f64;
}

/// Implemented in addition to [`Normalize`] when it's possible to calculate the maximum quantization error.
pub trait NormalizeError {
    /// Get the maximum amount of error that this normalization gives, given a certain quantization error.
    /// In other words, if a value is normalized and quantized
    /// and the result is then dequantized and denormalized, how large is the maximum error?
    fn max_error(quantization_error: f64) -> f64;
}
