# numquant

Quantize numbers to a smaller range to save bandwidth or memory.

The input floating point value is expected within a given range. Values outside this range will be clamped. The input value will then be quantized into a given integer range.

For example, given the allowed range -1000.0 to 1000.0, and the quantized range 0 to 255 (to fit in a byte), the value -1000.0 would be quantized to 0, and 1000.0 would be quantized to 255, and values in-between are linearly interpolated between 0 and 255.

## Example

This example uses the type `Quantized<U8<0, 1000>>` that converts any floating point number between 0.0 and 1000.0 to a byte (which has the range 0 to 255). Some precision is lost, but an approximate value can be brought back.

```rust
use numquant::{Quantize, Value, Linear, quantizer::Quantizer};
let original = 500.0;
// Quantize the value into a byte between 0 and 255.
// Quantization supports inputs between 0 and 1000.
type T = Value::<Quantizer<u8, 255>, Linear<0, 1000>>;
let quantized = T::from_f64(original);
// Convert it back to an f64
let dequantized = quantized.to_f64();
// The conversion isn't lossless, but the dequantized value is close to the original:
approx::assert_abs_diff_eq!(original, dequantized, epsilon = T::max_error());
```

## Links

  * [Documentation](https://docs.rs/numquant)
  * [crates.io](https://crates.io/crates/numquant)
