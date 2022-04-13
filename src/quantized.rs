use std::marker::PhantomData;

use crate::Quantize;

/// Contains a quantized value.
///
/// ```
/// use numquant::{Quantized, IntRange};
/// let q = Quantized::<IntRange<u8, 0xff, 0, 1000>>::from_f64(500.0);
/// let v = q.to_f64();
/// approx::assert_abs_diff_eq!(v, 500.0, epsilon = 2.0);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quantized<Q: Quantize>(Q::Type, PhantomData<Q>);

impl<Q: Quantize> Quantized<Q>
where
    Q::Type: Into<f64>,
{
    pub fn set(&mut self, value: f64) {
        self.0 = Q::quantize(value);
    }
}

impl<Q: Quantize> Quantized<Q>
where
    Q::Type: Into<f64> + Clone,
{
    pub fn from_raw(v: Q::Type) -> Self {
        Self(v, Default::default())
    }

    pub fn raw(&self) -> Q::Type {
        self.0.clone()
    }

    /// Create an instance from an `f64`.
    pub fn from_f64(v: f64) -> Self
    where
        Self: Sized,
    {
        Self::from_raw(Q::quantize(v))
    }

    /// Dequantize the stored value into an `f64`.
    pub fn to_f64(&self) -> f64 {
        Q::dequantize(self.0.clone())
    }

    /// Create an instance from an `f32`.
    pub fn from_f32(v: f32) -> Self
    where
        Self: Sized,
    {
        Self::from_raw(Q::quantize(v as f64))
    }

    /// Dequantize the stored value into an `f32`.
    pub fn to_f32(&self) -> f32 {
        self.to_f64() as f32
    }
}

impl<Q: Quantize> Default for Quantized<Q>
where
    Q::Type: Default,
{
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

impl<Q: Quantize> Clone for Quantized<Q>
where
    Q::Type: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<Q: Quantize> std::fmt::Debug for Quantized<Q>
where
    Q::Type: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(std::any::type_name::<Self>())
            .field(&self.0)
            .finish()
    }
}
