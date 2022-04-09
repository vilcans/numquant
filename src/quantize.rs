/// Trait for quantizing and dequantizing values.
pub trait Quantize {
    /// The type to store the quantized value in.
    type Type;

    /// Quantize a floating point value.
    fn quantize(value: f64) -> Self::Type;

    /// Restore the floating point value from a quantized value.
    fn dequantize(quantized: Self::Type) -> f64;

    /// Calculates the maximum quantization error.
    /// The absolute difference between the quantized and original value will not be greater than this,
    /// unless it has been clamped because it was out of range.
    fn max_error() -> f64;
}
