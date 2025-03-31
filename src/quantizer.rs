//! Defines the standard [`Quantize`] implementation [`Quantizer`].

use std::marker::PhantomData;

use crate::Quantize;

/// For quantizing values linearly to a range.
///
/// Accepts input values between 0.0 and 1.0.
/// 0.0 will be quantized to 0 of type `T`.
/// 1.0 will be quantized to `Q_MAX` of type `T`.
/// Values in-between are linearly interpolated between 0 and `Q_MAX`.
/// Values outside of the range 0.0..1.0 will be clamped to be inside the range.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Quantizer<T, const Q_MAX: u64> {
    _phantom: PhantomData<T>,
}

impl<T, const Q_MAX: u64> Quantize for Quantizer<T, Q_MAX>
where
    T: Into<f64> + TryFrom<u64> + Copy,
{
    type Type = T;

    fn quantize(value: f64) -> Self::Type {
        quantize(value, Self::q_max())
    }

    fn dequantize(quantized: Self::Type) -> f64 {
        dequantize(quantized, Self::q_max())
    }

    fn max_error() -> f64 {
        max_error(Self::q_max().into())
    }
}

impl<T, const Q_MAX: u64> Quantizer<T, Q_MAX>
where
    T: TryFrom<u64>,
{
    /// Maximum for the quantized value.
    /// The quantized value will be between 0 and this value (inclusive).
    pub fn q_max() -> T {
        match Q_MAX.try_into() {
            Ok(v) => v,
            Err(_) => panic!("Q_MAX={Q_MAX} not convertible to T"),
        }
    }
}

/// Linearly quantize a value.
pub fn quantize<T>(value: f64, q_max: T) -> T
where
    T: Into<f64> + TryFrom<u64> + Copy,
{
    let value = value.clamp(0.0, 1.0);
    let q_max: f64 = q_max.into();
    // Regarding the `min` call, see https://stackoverflow.com/a/600016
    let v = ((q_max + 1.0) * value).min(q_max) as u64;
    match v.try_into() {
        Ok(v) => v,
        Err(_) => panic!("{v} not convertible to T"),
    }
}

/// Linearly dequantize a value.
pub fn dequantize<T>(quantized: T, q_max: T) -> f64
where
    T: Into<f64> + Copy,
{
    let quantized = quantized.into();
    let q_max = q_max.into();
    quantized / q_max
}

/// The maximum error when quantizing linearly.
/// The absolute difference between the quantized and original value will not be greater than this,
/// unless it has been clamped because it was out of range.
pub fn max_error(q_max: f64) -> f64 {
    1.0 / (q_max + 1.0)
}

/// Stores the quantized value in a `u8` and uses the full range of `u8` when quantizing.
pub type U8 = Quantizer<u8, { u8::MAX as u64 }>;
/// Stores the quantized value in a `u16` and uses the full range of `u16` when quantizing.
pub type U16 = Quantizer<u16, { u16::MAX as u64 }>;
/// Stores the quantized value in a `u32` and uses the full range of `u32` when quantizing.
pub type U32 = Quantizer<u32, { u32::MAX as u64 }>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn linear_quantize() {
        assert_eq!(Quantizer::<u16, 1000>::quantize(0.0), 0u16);
        assert_eq!(Quantizer::<u16, 1000>::quantize(0.5), 500u16);
        assert_eq!(Quantizer::<u16, 1000>::quantize(1.0), 1000u16);
    }

    #[test]
    fn linear_dequantize() {
        assert_eq!(Quantizer::<u16, 1000>::dequantize(0u16), 0.0);
        assert_eq!(Quantizer::<u16, 1000>::dequantize(500u16), 0.5);
        assert_eq!(Quantizer::<u16, 1000>::dequantize(1000u16), 1.0);
    }

    #[test]
    fn u8_type() {
        assert_eq!(U8::quantize(0.0), 0x00u8);
        assert_eq!(U8::quantize(1.0), 0xffu8);
    }

    #[test]
    fn u16_type() {
        assert_eq!(U16::quantize(0.0), 0x0000u16);
        assert_eq!(U16::quantize(1.0), 0xffffu16);
    }
    #[test]
    fn u32_type() {
        assert_eq!(U32::quantize(0.0), 0x00000000u32);
        assert_eq!(U32::quantize(1.0), 0xffffffffu32);
    }
}
