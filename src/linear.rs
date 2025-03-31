//! For normalizing values linearly to the range 0.0..1.0.

use crate::{
    normalize::{Normalize, NormalizeError},
    range_arg::RangeArg,
};

/// Normalizes/denormalizes a value linearly between MIN and MAX.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Linear<const MIN: RangeArg, const MAX: RangeArg>;

impl<const MIN: RangeArg, const MAX: RangeArg> Normalize for Linear<MIN, MAX> {
    fn normalize(value: f64) -> f64 {
        ((value - MIN as f64) / (MAX - MIN) as f64).clamp(0.0, 1.0)
    }

    fn denormalize(n: f64) -> f64 {
        (MAX - MIN) as f64 * n + MIN as f64
    }
}

impl<const MIN: RangeArg, const MAX: RangeArg> NormalizeError for Linear<MIN, MAX> {
    fn max_error(quantization_error: f64) -> f64 {
        quantization_error * (MAX - MIN) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize() {
        assert_eq!(Linear::<0, 100>::normalize(25.0), 0.25);
    }

    #[test]
    fn normalize_extremes() {
        assert_eq!(Linear::<-100, 100>::normalize(-100.0), 0.0);
        assert_eq!(Linear::<-100, 100>::normalize(100.0), 1.0);
    }

    #[test]
    fn normalize_greater_than_range() {
        assert_eq!(Linear::<-100, 100>::normalize(1000.0), 1.0);
    }

    #[test]
    fn normalize_lower_than_range() {
        assert_eq!(Linear::<-100, 100>::normalize(-1000.0), 0.0);
    }
}
