# numquant

Converts a floating point number to a value of another (smaller) type.

A typical use case is to reduce bandwidth or storage space.

The input floating point value must be within a given range. Values outside this range will be clamped.

## Example

This example uses the type `Quantized<U8<0, 1000>>` that converts any floating point number between 0.0 and 1000.0 to a byte (which has the range 0 to 255). Some precision is lost, but an approximate value can be brought back.

```rust
let original = 500.0;
// Quantize the value into a byte.
// Quantization supports inputs between 0 and 1000.
let quantized = Quantized::<U8<0, 1000>>::from_f64(original);
// Convert it back to an f64
let dequantized = quantized.to_f64();
// The conversion isn't lossless, but the dequantized value is close to the original:
approx::assert_abs_diff_eq!(original, dequantized, epsilon = U8::<0, 1000>::max_error());
```
