//! Lossy conversion from floating point to a smaller integer type with a fixed range.
//!
//! ## Quantize an `f64` to a byte and back again
//!
//! ```
//! use numquant::int_types::Q8;
//! let original = 500.0;
//! // Quantize the value into a byte between 0 and 255.
//! // Quantization supports inputs between 0 and 1000.
//! type T = Q8::<0, 1000>;
//! let quantized = T::from_f64(original);
//! // Convert it back to an f64
//! let dequantized = quantized.to_f64();
//! // The conversion isn't lossless, but the dequantized value is close to the original:
//! approx::assert_abs_diff_eq!(original, dequantized, epsilon = T::max_error());
//! ```

pub mod int_types;
mod linear;
mod normalize;
mod quantize;
pub mod quantizer;
mod range_arg;
mod value;

pub use linear::Linear;
pub use quantize::Quantize;
pub use value::Value;
