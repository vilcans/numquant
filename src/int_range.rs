use std::{marker::PhantomData, ops::Range};

use crate::linear::Linear;

/// Quantizes/dequantizes to a value between 0 and `Q_MAX` stored in type `T`.
/// The range for the unquantized value is between `MIN` and `MAX`. Values outside of this are clamped.
pub struct IntRange<T, const Q_MAX: u64, const MIN: i64, const MAX: i64>(PhantomData<T>);

impl<T, const Q_MAX: u64, const MIN: i64, const MAX: i64> Linear for IntRange<T, Q_MAX, MIN, MAX>
where
    u64: TryInto<T>,
{
    type Type = T;

    fn range() -> Range<f64> {
        MIN as f64..MAX as f64
    }

    fn q_max() -> Self::Type {
        match Q_MAX.try_into() {
            Ok(v) => v,
            Err(_) => panic!("Q_MAX not convertible to T"),
        }
    }
}

/// Quantizes/dequantizes to a value stored in an `u8`, using the full range of the `u8`.
/// The range for the unquantized value is between `MIN` and `MAX`. Values outside of this are clamped.
pub type U8<const MIN: i64, const MAX: i64> = IntRange<u8, 0xff, MIN, MAX>;

/// Quantizes/dequantizes to a value stored in an `u16`, using the full range of the `u16`.
/// The range for the unquantized value is between `MIN` and `MAX`. Values outside of this are clamped.
pub type U16<const MIN: i64, const MAX: i64> = IntRange<u16, 0xffff, MIN, MAX>;

/// Quantizes/dequantizes to a value stored in an `u32`, using the full range of the `u32`.
/// The range for the unquantized value is between `MIN` and `MAX`. Values outside of this are clamped.
pub type U32<const MIN: i64, const MAX: i64> = IntRange<u32, 0xffffffff, MIN, MAX>;

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::quantize::Quantize;
    use crate::{Quantized, U8};

    #[test]
    fn test_byte() {
        type Type = U8<100, 150>;
        let value = 125.0;
        let v = Quantized::<Type>::from_f64(value);
        approx::assert_abs_diff_eq!(v.to_f64(), value, epsilon = Type::max_error());
    }

    #[test]
    fn quantize_values() {
        assert_eq!(U8::<0, 60>::quantize(0.0,), 0x00);
        assert_eq!(U8::<0, 60>::quantize(30.0,), 0x80);
        assert_eq!(U8::<0, 60>::quantize(60.0,), 0xff);
    }

    #[test]
    fn unquantize_values() {
        type Type = U8<0, 100>;
        assert_eq!(Type::dequantize(0x00), 0.0);
        assert_abs_diff_eq!(Type::dequantize(0x80), 50.0, epsilon = Type::max_error());
        assert_eq!(Type::dequantize(0xff), 100.0);
    }

    #[test]
    fn quantize_out_of_range_clamps() {
        assert_eq!(U8::<0, 100>::quantize(-1.0,), 0x00);
        assert_eq!(U8::<0, 100>::quantize(100.1,), 0xff);
    }
}
