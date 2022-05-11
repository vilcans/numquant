//! Defines some types that use the full range of the underlying primitive type.

use crate::{quantizer, range_arg::RangeArg, Linear, Value};

/// A value within a given range that is quantized to fit in a `u8`.
pub type Q8<const MIN: RangeArg, const MAX: RangeArg> = Value<quantizer::U8, Linear<MIN, MAX>>;

/// A value within a given range that is quantized to fit in a `u16`.
pub type Q16<const MIN: RangeArg, const MAX: RangeArg> = Value<quantizer::U16, Linear<MIN, MAX>>;

/// A value within a given range that is quantized to fit in a `u32`.
pub type Q32<const MIN: RangeArg, const MAX: RangeArg> = Value<quantizer::U16, Linear<MIN, MAX>>;
