//! Lossy conversion from floating point to a smaller integer type with a fixed range.
//!
//! ## Quantize an `f64` to a byte and back again
//! ```
//! use numquant::{Quantize, Quantized, U8};
//! let original = 500.0;
//! // Quantize the value into a byte.
//! // Quantization supports inputs between 0 and 1000.
//! let quantized = Quantized::<U8<0, 1000>>::from_f64(original);
//! // Convert it back to an f64
//! let dequantized = quantized.to_f64();
//! // The conversion isn't lossless, but the dequantized value is close to the original:
//! approx::assert_abs_diff_eq!(original, dequantized, epsilon = U8::<0, 1000>::max_error());
//! ```

mod int_range;
pub mod linear;
mod quantize;
mod quantized;

pub use int_range::{IntRange, U16, U32, U8};
pub use quantize::Quantize;
pub use quantized::Quantized;
